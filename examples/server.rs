use knife_framework::{
    crates::hyper::{Body, Request, Response},
    knife_router, knife_server,
    util::Result,
};

#[knife_server(project = "knife", application = "knife-sample")]
async fn main() {}

#[knife_router(path = "/hello", method = "get")]
async fn handler(req: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("hello world")))
}
