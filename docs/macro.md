# knife-macro宏定义

## 包含以下宏定义

* **knife_server:** 服务启动属性宏
```rust
#[knife_server(project = "knife", application = "knife-sample")]
fn main() {}
```

* **knife_component:** 全局容器属性宏
```rust
#[knife_component(name = "bean")]
pub struct Bean {}
```

* **knife_router:** 路由属性宏
```rust
#[knife_router(path="/")]
fn handler() -> &'static str {
    "Hello, world"
}
```

* **EnumName:** 枚举名称派生宏
```rust
#[derive(EnumName)]
pub(crate) enum Type {
    String(String),
    Number(i32),
}
```