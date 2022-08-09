use knife_macro::knife_component;
use knife_util::futures::{
    channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    executor::{block_on, ThreadPool},
    task::FutureObj,
    Future, StreamExt,
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
        worker_number: usize,
        thread_name: &'static str,
        action: FutureObj<'static, ()>,
    },
}

impl std::fmt::Debug for BootstrapEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NewThread {
                worker_number,
                thread_name,
                action: _,
            } => f
                .debug_struct("NewThread")
                .field("worker_number", worker_number)
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
            channel: unbounded::<BootstrapEvent>(),
        }
    }
    pub(crate) fn start<F>(&mut self, start_type: &'static str, f: F) -> &Self
    where
        F: Fn() + Send + 'static,
    {
        debug!("准备启动程序");
        let (_s, r) = &mut self.channel;
        block_on(async move {
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
            launch_application().await;

            loop {
                let msg = r.next().await.unwrap();
                Bootstrap::on_receive_event(msg).await;
            }
        });
        self
    }

    async fn on_receive_event(event: BootstrapEvent) {
        debug!("接受到事件BootstrapEvent {:?}", event);
        match event {
            BootstrapEvent::NewThread {
                worker_number,
                thread_name,
                action,
            } => {
                let rt = ThreadPool::builder()
                    .pool_size(worker_number.clone())
                    .create()
                    .expect(format!("构建线程[{}]失败", thread_name).as_str());
                let _res = rt.spawn_obj_ok(action);
            }
        }
    }

    pub(crate) fn new_thread<F>(worker_number: usize, thread_name: &'static str, callback: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let bootstrap = Bootstrap::get_instance() as &mut Bootstrap;
        bootstrap
            .channel
            .0
            .unbounded_send(BootstrapEvent::NewThread {
                worker_number,
                thread_name,
                action: FutureObj::new(Box::new(callback)),
            })
            .unwrap();
    }
}
