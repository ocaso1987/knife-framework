use knife_framework::{add_config, knife_router, knife_server, util::Result};
use knife_util::hyper::{Body, Request, Response};

#[knife_server(project = "knife", application = "knife-sample")]
async fn main() {
    add_config(
        r#"
            knife:
                web_server:
                    port: 8080
        "#,
    );
}

#[knife_router(path = "/hello", method = "get")]
async fn handler(req: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("hello world")))
}
