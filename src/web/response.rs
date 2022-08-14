use knife_util::{
    hyper::{Body, Response},
    AnyError, Result,
};

pub struct HyperResponse {
    resp: Option<Response<Body>>,
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
