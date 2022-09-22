//! 应用程序模块
use knife_macro::knife_component;
use knife_util::{
    any::{AnyFuture, AnyHandler},
    crates::{
        futures::{join, select, FutureExt},
        tokio::{
            self,
            sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        },
    },
    Result, OK,
};
use tracing::debug;

use std::{collections::HashMap, future::Future};

use crate::{
    app::{config::Config, logger::Logger},
    boot::bootstrap::Bootstrap,
    db::Db,
    web::server::Web,
};

/// 应用事件
pub struct ApplicationEvent {
    /// 应用事件名称
    name: String,
    /// 应用事件参数
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

/// 应用事件：已初始化，默认main方法将在此时进行处理
pub const EVENT_INIT: &str = "INIT";
/// 应用事件：已加载配置
pub const EVENT_CONFIG: &str = "CONFIG";
/// 应用事件：已完成应用模块加载
pub const EVENT_LAUNCH: &str = "LAUNCH";
/// 应用事件：已完成应用准备工作
pub const EVENT_READY: &str = "READY";
/// 应用事件：已启动
pub const EVENT_STARTED: &str = "STARTED";

/// 应用程序模块
///
/// 应用程序模块运行在独立线程，内含一组消息通道和一个消息回调方法集合
/// 当消息通道接受到应用消息时，会调用是否有默认消息回调方法进行处理
/// 这个一个简易的事件传递机制，后续会按生命周期继续优化重构
#[knife_component(
    name = "GLOBAL_APPLICATION",
    generate_method = "new",
    crate_builtin_name = "crate"
)]
pub struct Application {
    /// 消息通道，接收应用消息
    channel: (
        UnboundedSender<ApplicationEvent>,
        UnboundedReceiver<ApplicationEvent>,
    ),
    /// 应用回调，处理应用消息
    handlers: HashMap<String, AnyHandler<'static, ApplicationEvent, ()>>,
}

impl Application {
    /// 应用模块构造器
    pub(crate) fn new() -> Self {
        Application {
            channel: unbounded_channel::<ApplicationEvent>(),
            handlers: HashMap::new(),
        }
    }

    /// 处理应用消息
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

/// 初始化应用模块处理线程
pub async fn launch_application() -> Result<()> {
    debug!("线程ApplicationThread初始化...");
    let app = Application::get_instance() as &mut Application;
    let (_s, r) = &mut app.channel;

    set_application_hook(EVENT_CONFIG, |_| async move {
        Logger::launch().await;
        Config::launch().await;
        Logger::reload().await;
        send_application_event(EVENT_LAUNCH, vec![]);
    });
    set_application_hook(EVENT_LAUNCH, |_| async move {
        let _ = join!(Db::launch(), Web::launch());
        async move {
            send_application_event(EVENT_READY, vec![]);
        }
        .await;
    });

    Bootstrap::new_thread("ApplicationThread", async move {
        tokio::spawn(async {
            send_application_event(EVENT_INIT, vec![]);
        });
        loop {
            select! {
                msg = r.recv().fuse() =>  Application::on_receive_event(msg.unwrap()).await,
            }
        }
    });
    OK(())
}

/// 发送应用消息，并交由应用模块线程进行处理
pub(crate) fn send_application_event(event_name: &'static str, event_params: Vec<String>) {
    let app = Application::get_instance() as &mut Application;
    app.channel
        .0
        .send(ApplicationEvent {
            name: event_name.to_string(),
            param: event_params,
        })
        .unwrap();
}

/// 设置应用处理回调，以处理应用消息
pub fn set_application_hook<F, R>(event_name: &'static str, hook: F)
where
    F: (FnOnce(ApplicationEvent) -> R) + Send + 'static,
    R: Future<Output = ()> + Send + 'static,
{
    let app = Application::get_instance() as &mut Application;
    app.handlers.insert(
        event_name.to_string(),
        AnyHandler::new(Box::new(|msg| {
            AnyFuture::new(Box::new(async move { hook(msg).await }))
        })),
    );
}
