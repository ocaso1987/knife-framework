use knife_macro::knife_component;
use rbatis::rbatis::Rbatis;
use rbdc_pg::driver::PgDriver;
use tracing::debug;

use crate::app::config::app_setting;

#[knife_component(
    name = "GLOBAL_DB",
    generate_method = "new",
    async_init = "init",
    crate_builtin_name = "crate"
)]
pub struct Db {
    pub(crate) rb: Rbatis,
}

impl Db {
    pub(crate) fn new() -> Self {
        Db { rb: Rbatis::new() }
    }

    pub async fn init(&self) {
        let setting = app_setting();
        let driver_url = setting.knife.db.driver_url.to_string();
        if !driver_url.is_empty() {
            debug!("连接数据源:{}", driver_url);
            self.rb
                .link(PgDriver {}, driver_url.as_str())
                .await
                .unwrap();
        }
    }

    pub(crate) async fn launch() {
        let _ = Db::get_instance_async().await as &mut Db;
    }
}

pub fn rb() -> &'static mut Rbatis {
    let db = Db::get_instance() as &mut Db;
    &mut db.rb
}
