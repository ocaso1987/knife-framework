use knife_macro::knife_component;
use knife_util::{
    crates::sqlx::{Database, Pool, Postgres},
    AnyValue, Result,
};
use tracing::debug;

use super::config::app_setting;

#[knife_component(
    name = "GLOBAL_DB",
    generate_method = "new",
    async_init = "init",
    crate_builtin_name = "crate"
)]
pub struct Db {
    pub(crate) db: Option<AnyValue>,
}

impl Db {
    pub(crate) fn new() -> Self {
        Db { db: None }
    }

    pub async fn init(&mut self) {
        let setting = app_setting();
        let driver_url = setting.knife.db.driver_url.to_string();
        if !driver_url.is_empty() {
            debug!("连接数据源:{}", driver_url);
            let pool = Pool::<Postgres>::connect(driver_url.as_str())
                .await
                .unwrap();
            self.db.replace(AnyValue::new(pool));
        }
    }

    pub(crate) async fn launch() -> Result<()> {
        let _ = Db::get_instance_async().await as &mut Db;
        Ok(())
    }
}

pub fn db<T>() -> &'static mut Pool<T>
where
    T: Database,
{
    let db = Db::get_instance() as &mut Db;
    db.db.as_ref().unwrap().as_mut::<Pool<T>>()
}
