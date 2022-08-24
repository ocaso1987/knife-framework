use knife_macro::knife_component;
use knife_util::Result;
use rbatis::rbatis::Rbatis;
use rbdc_pg::driver::PgDriver;
use tracing::debug;

use super::config::app_setting;

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

    pub(crate) async fn launch() -> Result<()> {
        let _ = Db::get_instance_async().await as &mut Db;
        Ok(())
    }
}

pub fn rb() -> &'static mut Rbatis {
    let db = Db::get_instance() as &mut Db;
    &mut db.rb
}

#[macro_export]
macro_rules! rb_tx {
    () => {
        rb().acquire_begin()
            .await
            .unwrap()
            .defer_async(|mut tx__1| async move {
                if !tx__1.is_done() {
                    tx__1.rollback().await.unwrap();
                }
            })
    };
}
