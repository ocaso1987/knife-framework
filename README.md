# knife-framework
Knife微服务框架

[![Crates.io](https://img.shields.io/crates/v/knife-framework)](https://crates.io/crates/knife-framework)
[![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/ocaso1987/knife-framework/knife-framework)](https://github.com/ocaso1987/knife-framework)
[![docs.rs](https://img.shields.io/docsrs/knife-framework)](https://docs.rs/knife-framework)

## 框架介绍

本框架旨在构建一个类似于SpringBoot的微服务框架，并支持对企业级配置注册等资源进行统一访问与管理。

## 文档

说明文档请参考：
[说明文档](https://ocaso1987.github.io/knife-framework/)

## 从一个示例开始

```rust
use knife_framework::{
    crates::hyper::{Body, Request, Response},
    knife_router, knife_server,
    util::{Result, OK},
};

#[knife_server(project = "knife", application = "knife-sample")]
fn main() {
}

#[knife_router(path = "/hello", method = "get")]
async fn handler(req: Request<Body>) -> Result<Response<Body>> {
    OK(Response::new(Body::from("hello world")))
}
```

## 依赖

本框架依赖的组件大多均通过Reexport方式导出，无需另行依赖，但部分组件除外，需要另行引用，主要包括:

```toml
[dependencies]
knife-framework = "0.1.x"
serde = "1.0.144"
tracing = "0.1.36"
```

你可以克隆该项目，并且执行项目中示例： cargo run --example example_name.
