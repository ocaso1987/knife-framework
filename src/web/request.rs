use knife_util::hyper::{Body, Request};

pub struct HyperRequest {
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
