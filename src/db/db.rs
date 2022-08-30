//! 数据库初始化模块
use knife_macro::knife_component;
use knife_util::{
    crates::sqlx::{Database, Pool, Postgres},
    AnyValue, Result,
};
use tracing::debug;

use crate::app_setting;

/// 数据库初始模块结构体
///
/// 内部采用sqlx数据源
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
    /// 数据库模块构造器
    pub(crate) fn new() -> Self {
        Db { db: None }
    }

    /// 数据库连接初始化
    pub async fn init(&mut self) {
        let setting = app_setting();
        let database_url = setting.knife.db.database_url.to_string();
        if !database_url.is_empty() {
            debug!("连接数据源:{}", database_url);
            let pool = Pool::<Postgres>::connect(database_url.as_str())
                .await
                .unwrap();
            self.db.replace(AnyValue::new(pool));
        }
    }

    /// 加载数据库模块
    pub(crate) async fn launch() -> Result<()> {
        let _ = Db::get_instance_async().await as &mut Db;
        Ok(())
    }
}

/// 获取数据库连接
pub fn db_conn<T>() -> &'static mut Pool<T>
where
    T: Database,
{
    let db = Db::get_instance() as &mut Db;
    db.db.as_ref().unwrap().as_mut::<Pool<T>>()
}
