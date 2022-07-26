//! 日志模块
use knife_macro::knife_component;
use knife_util::crates::tracing_subscriber::{
    self,
    prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt, // , Layer,
};

/// 日志模块
#[knife_component(name = "GLOBAL_LOGGER", crate_builtin_name = "crate")]
pub struct Logger {}

impl Logger {
    pub(crate) async fn launch() {
        let logger = Logger::get_instance() as &mut Logger;
        logger.load_default();
    }

    pub fn load_default(&mut self) {
        // let application_id = std::env::var("knife_application_id").unwrap();
        // let tracer = opentelemetry_jaeger::new_agent_pipeline()
        //     .with_service_name(application_id)
        //     .install_simple()
        //     .unwrap();
        // let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        let stdout = tracing_subscriber::fmt::layer().with_writer(std::io::stdout);
        tracing_subscriber::registry()
            // .with(opentelemetry)
            .with(stdout)
            .try_init()
            .unwrap();
    }

    pub(crate) async fn reload() {}
}
