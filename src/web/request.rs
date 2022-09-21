use knife_util::{
    bean::AsValueTrait,
    crates::{
        async_trait::async_trait,
        hyper::{body::to_bytes, Body, Request},
    },
    crates_builtin::serde_json,
    future::AsyncInto,
    Value,
};
use tracing::info;

/// 请求对象
///
/// 现fn回调的参数会自动转换为hyper的request进行处理
pub struct HyperRequest {
    /// 请求参数
    req: Request<Body>,
    /// 路由
    router_name: String,
}

#[async_trait]
impl AsyncInto<Request<Body>> for HyperRequest {
    async fn async_into(self) -> Request<Body> {
        info!("接受Web请求[{}],数据:$binary_data", self.router_name);
        self.req
    }
}

#[async_trait]
impl AsyncInto<Value> for HyperRequest {
    async fn async_into(self) -> Value {
        let req_bytes = to_bytes(self.req.into_body()).await.unwrap();
        let req_obj: serde_json::Value = serde_json::from_slice(&req_bytes).unwrap();
        info!("接受Web请求[{}],数据:{}", self.router_name, &req_obj);
        req_obj.as_value().unwrap()
    }
}

impl HyperRequest {
    pub(crate) fn new(req: Request<Body>, router_name: String) -> HyperRequest {
        HyperRequest { req, router_name }
    }
}
