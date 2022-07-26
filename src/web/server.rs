use std::collections::HashMap;

use knife_macro::knife_component;
use knife_util::{
    any::AnyRef,
    crates::hyper::{
        server::conn::AddrStream,
        service::{make_service_fn, service_fn},
        Body, Request, Response, Server, StatusCode,
    },
    error::{AppError, ERR_WEB},
    types::StringExt,
    Result, OK,
};
use tracing::{debug, warn};

use crate::{
    app_setting, boot::bootstrap::Bootstrap, component_global, foreach_global, Component,
    HyperRequest,
};

/// Web服务模块
#[knife_component(name = "GLOBAL_WEB", crate_builtin_name = "crate")]
pub struct Web {
    /// 路由对象缓存
    ///
    /// 如果路由在此处命中，则无需从全局容器中查找合适的路由
    router_map: HashMap<String, AnyRef>,
}

impl Web {
    /// 路由模块初始化
    pub async fn launch() {}

    /// 路由模块启动
    ///
    /// 发送消息到Bootstrap模块以启动线程
    pub async fn start() {
        Bootstrap::new_thread("WebServerThread", async move {
            debug!("线程WebServerThread初始化...");
            Self::start_instance().await;
        });
    }

    /// 路由模块启动
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

/// 匹配路由请求
async fn match_req(req: Request<Body>) -> Result<Response<Body>> {
    let router_name = &format!(
        "{}:{}",
        req.method().to_string().to_uppercase(),
        req.uri().path()
    );
    let router = get_match_router(router_name.as_str());
    if let Some(c) = router {
        let req = HyperRequest::new(req, router_name.to_string());
        let res = c.as_router().router_handle(req).await;
        let resp: Result<Response<Body>> = res.into();
        if resp.is_ok() {
            resp
        } else {
            let err = resp.err().unwrap();
            warn!("全局异常:{:?}", err);
            OK(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(err.to_json_string()))
                .unwrap())
        }
    } else {
        OK(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(
                ERR_WEB
                    .msg_detail(format!("没找到路由{}", router_name.clone()).as_str())
                    .to_json_string(),
            ))
            .unwrap())
    }
}

/// 获取匹配的路由
fn get_match_router(router_name: &str) -> Option<&'static mut Component> {
    let router = component_global("router".to_string(), router_name.to_string());
    if router.is_some() {
        return router;
    }
    {
        let web = Web::get_instance() as &'static mut Web;
        let router_map = &mut web.router_map as &'static mut HashMap<String, AnyRef>;
        if router_map.get(&router_name.to_string()).is_none() {
            foreach_global("router".to_string(), move |c| {
                if router_name.to_string().glob_match(c.0.to_string()) {
                    router_map.insert(router_name.to_string(), AnyRef::new(c.1));
                }
            });
        }
    }
    {
        let web = Web::get_instance() as &'static mut Web;
        let router_map = &mut web.router_map as &'static mut HashMap<String, AnyRef>;
        router_map
            .get(&router_name.to_string())
            .map(|x| x.to_mut::<Component>())
    }
}
