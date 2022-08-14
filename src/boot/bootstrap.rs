use std::future::Future;

use knife_macro::knife_component;
use knife_util::{
    tokio::{
        self,
        runtime::Runtime,
        select,
        sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    },
    FutureObj,
};
use tracing::debug;

use crate::{
    app::application::{
        launch_application, send_application_event, set_application_hook, EVENT_CONFIG, EVENT_INIT,
        EVENT_READY, EVENT_STARTED,
    },
    web::server::Web,
};
pub(crate) const SERVER: &'static str = "SERVER";

pub enum BootstrapEvent {
    NewThread {
        thread_name: &'static str,
        action: FutureObj<'static, ()>,
    },
}

impl std::fmt::Debug for BootstrapEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NewThread {
                thread_name,
                action: _,
            } => f
                .debug_struct("NewThread")
                .field("thread_name", thread_name)
                .finish(),
        }
    }
}

#[knife_component(
    name = "GLOBAL_BOOTSTRAP",
    generate_method = "new",
    crate_builtin_name = "crate"
)]
pub struct Bootstrap {
    channel: (
        UnboundedSender<BootstrapEvent>,
        UnboundedReceiver<BootstrapEvent>,
    ),
}

impl Bootstrap {
    pub(crate) fn new() -> Self {
        Bootstrap {
            channel: unbounded_channel::<BootstrapEvent>(),
        }
    }
    pub(crate) fn start<F>(&mut self, start_type: &'static str, f: F) -> &Self
    where
        F: Fn() + Send + 'static,
    {
        debug!("准备启动程序");
        let (_s, r) = &mut self.channel;

        set_application_hook(EVENT_INIT, |_| async move {
            f();
            send_application_event(EVENT_CONFIG, vec![]);
        });
        set_application_hook(EVENT_READY, move |_| async move {
            if start_type == SERVER {
                Web::start().await;
            } else {
                panic!("不支持的启动类型");
            }
            send_application_event(EVENT_STARTED, vec![]);
        });

        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            tokio::spawn(launch_application()); //正式启动
            loop {
                select! {
                    Some(msg) = r.recv() =>  Bootstrap::on_receive_event(msg).await,
                }
            }
        });
        self
    }

    async fn on_receive_event(event: BootstrapEvent) {
        debug!("接受到事件BootstrapEvent {:?}", event);
        match event {
            BootstrapEvent::NewThread {
                thread_name: _,
                action,
            } => {
                tokio::task::spawn(async move { action.await });
            }
        }
    }

    pub(crate) fn new_thread<F>(thread_name: &'static str, callback: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let bootstrap = Bootstrap::get_instance() as &mut Bootstrap;
        bootstrap
            .channel
            .0
            .send(BootstrapEvent::NewThread {
                thread_name,
                action: FutureObj::new(Box::new(callback)),
            })
            .unwrap();
    }
}
