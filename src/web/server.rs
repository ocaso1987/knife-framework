use knife_macro::knife_component;
use knife_util::hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use knife_util::tokio::runtime::Builder;
use knife_util::AppError;
use tracing::debug;

use crate::{app_setting, boot::bootstrap::Bootstrap, component_global};

#[knife_component(name = "GLOBAL_WEB", crate_builtin_name = "crate")]
pub struct Web {}

impl Web {
    pub async fn launch() {}
    pub async fn start() {
        Bootstrap::new_thread(1, "WebServerThread", async move {
            debug!("线程WebServerThread初始化...");
            let rt = Builder::new_multi_thread()
                .worker_threads(4)
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(Self::start_instance());
        });
    }
    pub async fn start_instance() {
        let port = app_setting().knife.web_server.port;
        if port != 0 {
            let addr = format!("127.0.0.1:{}", port).parse().unwrap();
            let new_service = make_service_fn(move |_socket: &AddrStream| async move {
                Ok::<_, AppError>(service_fn(move |req: Request<Body>| async move {
                    match_req(req).await
                }))
            });
            let server = Server::bind(&addr).serve(new_service);
            debug!("Web服务监听:{}", addr);
            server.await.unwrap();
        }
    }
}

async fn match_req(req: Request<Body>) -> Result<Response<Body>, AppError> {
    let router_name = format!(
        "{}:{}",
        req.method().to_string().to_uppercase().as_str(),
        req.uri().path()
    );
    let router = component_global("router".to_string(), router_name.to_string());
    if let Some(c) = router {
        let req = req.into();
        let res = c.to_router().router_handle(req).await;
        let resp = res.into();
        resp
    } else {
        panic!("没找到路由{}", router_name.clone());
    }
}
