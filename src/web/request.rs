use knife_util::crates::hyper::{Body, Request};

/// 请求对象
///
/// 现fn回调的参数会自动转换为hyper的request进行处理
pub struct HyperRequest {
    /// 请求参数
    req: Request<Body>,
}

impl From<Request<Body>> for HyperRequest {
    fn from(req: Request<Body>) -> Self {
        HyperRequest { req }
    }
}

impl Into<Request<Body>> for HyperRequest {
    fn into(self) -> Request<Body> {
        self.req
    }
}
