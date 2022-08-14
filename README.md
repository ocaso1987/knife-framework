# knife-framework
Knife微服务框架

## 框架介绍

本框架旨在构建一个类似于SpringBoot的微服务框架，并支持对企业级配置注册等资源进行统一访问与管理。

## 从一个示例开始

```rust
use hyper::{Body, Request, Response};
use knife_framework::{knife_router, knife_server, Result};

#[knife_server(project = "knife", application = "knife-sample")]
fn main() {
    handler__Holder::get_instance();
}

#[knife_router(path = "/hello", method = "get")]
async fn handler(req: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("hello world")))
}
```

## 依赖

本框架依赖的组件大多均通过Reexport方式导出，无需另行依赖，但部分组件除外，需要另行引用，主要包括:

```toml
[dependencies]
knife-framework = { git = "https://github.com/ocaso1987/knife-framework.git", branch = "v0.1.0-dev" }
serde = "1.0.142"
lazy_static = "1.4.0"
rbatis = "4.0.18"
```
你可以克隆该项目，并且执行项目中示例： cargo run --example example_name.
