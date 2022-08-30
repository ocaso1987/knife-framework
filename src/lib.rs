//! knife-framework微服务框架
pub(crate) mod app {
    pub(crate) mod application;
    pub(crate) mod config;
    pub(crate) mod logger;
    pub(crate) mod setting;
}
pub(crate) mod bean {
    pub(crate) mod component;
    pub(crate) mod main;
    pub(crate) mod scope;
}
pub(crate) mod boot {
    pub(crate) mod bootstrap;
    pub(crate) mod main;
}

/// Web模块
pub(crate) mod web {
    pub(crate) mod request;
    pub(crate) mod response;
    pub(crate) mod server;
}

/// 数据库模块
pub mod db {
    pub(crate) mod db;
    pub(crate) mod sql;
    pub use db::*;
    pub use sql::*;
}

/// 公共模型
pub mod model {
    pub(crate) mod common;
    pub use common::*;
}
// pub mod ext {}

/// Reexport
pub mod crates {
    pub use knife_util::crates::*;
}

pub use knife_macro::*;

/// Reexport
pub mod util {
    pub use knife_util::*;
}

pub use app::config::{add_config, app_raw_setting, app_setting};
pub use bean::{
    component::{Component, ComponentTrait, RouterTrait},
    main::{component_global, foreach_global, get_global, register_global},
};
pub use boot::main::{start_server, stop_server};
pub use web::{request::HyperRequest, response::HyperResponse};
