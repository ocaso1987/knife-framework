//! 应用启动模块
use std::future::Future;

use knife_macro::knife_component;
use knife_util::{
    any::AnyFuture,
    crates::{
        futures::{select, FutureExt},
        tokio::{
            self,
            runtime::Runtime,
            sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        },
    },
};
use tracing::debug;

use crate::{
    app::application::{
        launch_application, send_application_event, set_application_hook, EVENT_CONFIG, EVENT_INIT,
        EVENT_READY, EVENT_STARTED,
    },
    web::server::Web,
};
pub(crate) const SERVER: &str = "SERVER";

/// 应用启动事件
pub enum BootstrapEvent {
    /// 新建线程事件
    NewThread {
        thread_name: &'static str,
        action: AnyFuture<'static, ()>,
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

/// 应用启动模块
///
/// 内部包含一个消息通道以处理应用启动事件
#[knife_component(
    name = "GLOBAL_BOOTSTRAP",
    generate_method = "new",
    crate_builtin_name = "crate"
)]
pub struct Bootstrap {
    /// 消息通道
    channel: (
        UnboundedSender<BootstrapEvent>,
        UnboundedReceiver<BootstrapEvent>,
    ),
}

impl Bootstrap {
    /// 启动模块构造器
    pub(crate) fn new() -> Self {
        Bootstrap {
            channel: unbounded_channel::<BootstrapEvent>(),
        }
    }

    /// 启动模块进行启动
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
                    msg = r.recv().fuse() =>  Bootstrap::on_receive_event(msg.unwrap()).await,
                }
            }
        });
        self
    }

    /// 处理启动事件
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

    /// 发送新建线程的启动事件，并在线程启动后进行回调
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
                action: AnyFuture::new(Box::new(callback)),
            })
            .unwrap();
    }
}
