use knife_util::{
    crates::hyper::{Body, Response},
    AnyError, Result,
};

/// 响应对象
///
/// 现fn回调的结果会自动转换为hyper的response进行处理
pub struct HyperResponse {
    /// 响应结果
    resp: Option<Response<Body>>,
    /// 响应异常
    err: Option<AnyError>,
}

impl From<Result<Response<Body>>> for HyperResponse {
    fn from(res: Result<Response<Body>>) -> Self {
        if res.is_ok() {
            HyperResponse {
                resp: res.ok(),
                err: None,
            }
        } else {
            HyperResponse {
                resp: None,
                err: res.err(),
            }
        }
    }
}

impl Into<Result<Response<Body>>> for HyperResponse {
    fn into(self) -> Result<Response<Body>> {
        if self.resp.is_some() {
            Ok(self.resp.unwrap())
        } else {
            Err(self.err.unwrap())
        }
    }
}
