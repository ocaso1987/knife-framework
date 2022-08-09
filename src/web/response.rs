use knife_util::{
    hyper::{Body, Response},
    AppError,
};

pub struct HyperResponse {
    resp: Option<Response<Body>>,
    err: Option<AppError>,
}

impl From<Result<Response<Body>, AppError>> for HyperResponse {
    fn from(res: Result<Response<Body>, AppError>) -> Self {
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

impl Into<Result<Response<Body>, AppError>> for HyperResponse {
    fn into(self) -> Result<Response<Body>, AppError> {
        if self.resp.is_some() {
            Ok(self.resp.unwrap())
        } else {
            Err(self.err.unwrap())
        }
    }
}
