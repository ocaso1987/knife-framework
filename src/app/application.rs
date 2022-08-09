use knife_macro::knife_component;
use knife_util::futures::{
    channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    join,
    task::FutureObj,
    Future, FutureExt, StreamExt,
};
use tracing::debug;

use std::collections::HashMap;

use crate::{
    app::{config::Config, db::Db, logger::Logger},
    boot::bootstrap::Bootstrap,
    util::Handler,
    web::server::Web,
};

pub struct ApplicationEvent {
    name: String,
    param: Vec<String>,
}

impl std::fmt::Debug for ApplicationEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApplicationEvent")
            .field("name", &self.name)
            .field("param", &self.param)
            .finish()
    }
}

pub const EVENT_INIT: &'static str = "INIT";
pub const EVENT_CONFIG: &'static str = "CONFIG";
pub const EVENT_LAUNCH: &'static str = "LAUNCH";
pub const EVENT_READY: &'static str = "READY";
pub const EVENT_STARTED: &'static str = "STARTED";

#[knife_component(
    name = "GLOBAL_APPLICATION",
    generate_method = "new",
    crate_builtin_name = "crate"
)]
pub struct Application {
    channel: (
        UnboundedSender<ApplicationEvent>,
        UnboundedReceiver<ApplicationEvent>,
    ),
    handlers: HashMap<String, Handler<'static, ApplicationEvent, ()>>,
}

impl Application {
    pub(crate) fn new() -> Self {
        Application {
            channel: unbounded::<ApplicationEvent>(),
            handlers: HashMap::new(),
        }
    }

    pub(crate) async fn on_receive_event(msg: ApplicationEvent) {
        debug!("接受到事件{:?}", msg);
        let app = Application::get_instance() as &mut Application;
        if app.handlers.contains_key(&msg.name) {
            let handler = app.handlers.get(&msg.name).unwrap();
            handler.invoke(msg).await
        } else {
            debug!("无法处理事件[{:?}],忽略之", &msg.name);
        }
    }
}

pub async fn launch_application() {
    Bootstrap::new_thread(1, "ApplicationThread", async move {
        debug!("线程ApplicationThread初始化...");
        let app = Application::get_instance() as &mut Application;

        set_application_hook(EVENT_CONFIG, |_| async move {
            let _ = Logger::launch()
                .then(|_| Config::launch())
                .then(|_| Logger::reload())
                .then(|_| async move {
                    send_application_event(EVENT_LAUNCH, vec![]);
                })
                .await;
        });
        set_application_hook(EVENT_LAUNCH, |_| async move {
            let _ = join!(Db::launch(), Web::launch());
            let _ = async move {
                send_application_event(EVENT_READY, vec![]);
            }
            .await;
        });
        send_application_event(EVENT_INIT, vec![]);

        let (_s, r) = &mut app.channel;
        loop {
            let msg = r.next().await.unwrap();
            Application::on_receive_event(msg).await;
        }
    });
}

pub(crate) fn send_application_event(event_name: &'static str, event_params: Vec<String>) {
    let app = Application::get_instance() as &mut Application;
    app.channel
        .0
        .unbounded_send(ApplicationEvent {
            name: event_name.to_string(),
            param: event_params,
        })
        .unwrap();
}

pub fn set_application_hook<F, R>(event_name: &'static str, hook: F)
where
    F: (FnOnce(ApplicationEvent) -> R) + Send + 'static,
    R: Future<Output = ()> + Send + 'static,
{
    let app = Application::get_instance() as &mut Application;
    app.handlers.insert(
        event_name.to_string(),
        Handler::new(Box::new(|msg| {
            FutureObj::new(Box::new(async move { hook(msg).await }))
        })),
    );
}
