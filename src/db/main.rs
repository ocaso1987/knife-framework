//! 数据库初始化模块
use knife_macro::knife_component;
use knife_util::crates::rbatis::Rbatis;
use tracing::debug;

use crate::{app_setting, crates::rbdc_pg::driver::PgDriver};

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
    pub(crate) db: Rbatis,
}

impl Db {
    /// 数据库模块构造器
    pub(crate) fn new() -> Self {
        Db { db: Rbatis::new() }
    }

    /// 数据库连接初始化
    pub async fn init(&mut self) {
        let setting = app_setting();
        let database_url = setting.knife.db.database_url.to_string();
        if !database_url.is_empty() {
            debug!("连接数据源:{}", database_url);
            self.db.init(PgDriver {}, database_url.as_str()).unwrap();
        }
    }

    /// 加载数据库模块
    pub(crate) async fn launch() {
        let _ = Db::get_instance_async().await as &mut Db;
    }
}

pub async fn get_db() -> &'static Rbatis {
    let db = Db::get_instance() as &mut Db;
    &db.db
}

// pub async fn with_tx<T, R, Z>(r: R) -> Result<Z>
// where
//     T: Database,
//     R: Future<Output = Result<Z>> + Send + 'static,
//     Z: Send,
// {
//     let db = Db::get_instance() as &mut Db;
//     let t_type = type_name::<T>();
//     if t_type == "Postgres" {
//         let util = db.db_util.as_ref().unwrap().as_ref::<PostgresDbUtil>();
//         let tx = PostgresDbUtil::get_tx().await;
//         let res = r.await;
//         if res.is_ok() {
//             tx.commit().await;
//         } else {
//             tx.rollback().await;
//         }
//         return res;
//     } else {
//         panic!("不支持的数据类型");
//     }
// }
