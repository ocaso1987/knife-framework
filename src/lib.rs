#[macro_use]
extern crate lazy_static;

pub(crate) mod app {
    pub(crate) mod application;
    pub(crate) mod config;
    pub(crate) mod db;
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
pub mod web {
    pub(crate) mod request;
    pub(crate) mod response;
    pub(crate) mod server;
}
pub mod model {
    pub(crate) mod common;
    pub use common::*;
}
pub mod ext {
    pub(crate) mod error;
    pub(crate) mod page;
    pub use error::from_rbatis;
    pub use page::*;
}

pub mod crates {
    pub use knife_util::crates::*;
    pub use tracing;
}

pub use knife_macro::*;
pub mod util {
    pub use knife_util::*;
}

pub use app::{
    config::{add_config, app_raw_setting, app_setting},
    db::rb,
};
pub use bean::{
    component::{Component, ComponentTrait, RouterTrait},
    main::{component_global, foreach_global, get_global, register_global},
};
pub use boot::main::{start_server, stop_server};
pub use web::{request::HyperRequest, response::HyperResponse};
