use std::collections::BTreeMap;

use knife_util::{
    bean::FromValueTrait,
    context::ContextTrait,
    crates::rbatis::rbdc::db::ExecResult,
    error::ERR_DB_DATA,
    iter::CollectResultTrait,
    page::{get_offset, PageResult},
    template::render_sql_template,
    types::StringExt,
    Result, Value, OK,
};
use serde::Deserialize;
use tracing::info;

use super::get_db;
use crate::crates::rbs;

pub async fn select_value(sql: &str, param: &Value) -> Result<rbs::Value> {
    match select_row(sql, param).await {
        Ok(Some(v2)) => {
            if v2.is_map() {
                let map = v2.as_map().unwrap();
                match map.first() {
                    Some((_k3, v3)) => OK(v3.clone()),
                    None => {
                        Err(ERR_DB_DATA.msg_detail("返回数据格式错误，期望单条数据，实际返回0条"))
                    }
                }
            } else if v2.is_array() {
                Err(ERR_DB_DATA.msg_detail("返回数据格式错误，期望单条数据，实际返回多条"))
            } else {
                OK(v2)
            }
        }
        Ok(None) => Err(ERR_DB_DATA.msg_detail("返回数据格式错误，期望单条数据，实际返回0条")),
        Err(e) => Err(e),
    }
}

pub async fn select_value_to_primitive<T>(sql: &str, param: &Value) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    select_value(sql, param)
        .await
        .map(|x| rbs::from_value::<T>(x).unwrap())
}

pub async fn select_row(sql: &str, param: &Value) -> Result<Option<rbs::Value>> {
    let param = &check_param_for_query(param);
    let sql = render_sql_template(sql.to_string(), param)?;
    let sql_param = sql
        .1
        .iter()
        .map(|x| rbs::Value::from_value(x).unwrap())
        .collect::<Vec<rbs::Value>>();
    info!("Sql语句: {}", &sql.0);
    info!("Sql参数: {:?}", &sql_param);
    let res = get_db().await.fetch(sql.0.as_str(), sql_param).await;
    match res {
        Ok(v) => {
            if v.is_array() {
                let arr = v.as_array().unwrap();
                if arr.len() == 1 {
                    let res = arr.get(0).unwrap().clone();
                    info!("Sql命中: {}", &res);
                    OK(Some(res))
                } else if arr.is_empty() {
                    info!("Sql命中: 0条");
                    OK(None)
                } else {
                    let err = ERR_DB_DATA.msg_detail("返回数据为多条");
                    info!("Sql异常: {:?}", err);
                    Err(err)
                }
            } else {
                let err = ERR_DB_DATA.msg_detail("返回数据格式错误");
                info!("Sql异常: {:?}", err);
                Err(err)
            }
        }
        Err(e) => {
            let err = e.into();
            info!("Sql异常: {:?}", err);
            Err(err)
        }
    }
}

pub async fn select_row_to_entity<T>(sql: &str, param: &Value) -> Result<Option<T>>
where
    T: for<'de> Deserialize<'de>,
{
    match select_row(sql, param).await {
        Ok(Some(v)) => match rbs::from_value::<T>(v) {
            Ok(v2) => OK(Some(v2)),
            Err(e) => Err(e.into()),
        },
        Ok(None) => OK(None),
        Err(e) => Err(e),
    }
}

pub async fn select_all(sql: &str, param: &Value) -> Result<Vec<rbs::Value>> {
    let param = &check_param_for_query(param);
    let sql = render_sql_template(sql.to_string(), param)?;
    let sql_param = sql
        .1
        .iter()
        .map(|x| rbs::Value::from_value(x).unwrap())
        .collect::<Vec<rbs::Value>>();
    info!("Sql语句: {}", &sql.0);
    info!("Sql参数: {:?}", &sql_param);
    let res = get_db()
        .await
        .fetch(sql.0.as_str(), sql_param)
        .await
        .map(|x| -> Vec<rbs::Value> {
            let res: Vec<rbs::Value> = if x.is_array() {
                x.as_array().unwrap().to_vec()
            } else {
                vec![x]
            };
            info!("Sql命中: {:?}条", res.len());
            res
        })
        .map_err(|e| {
            let err = e.into();
            info!("Sql异常: {:?}", err);
            err
        });
    res
}

pub async fn select_all_to_entity<T>(sql: &str, param: &Value) -> Result<Vec<T>>
where
    T: for<'de> Deserialize<'de>,
{
    match select_all(sql, param).await {
        Ok(v) => v
            .iter()
            .map(|x| rbs::from_value::<T>(x.clone()).map_err(|e| e.into()))
            .collect_into_vec(),
        Err(e) => Err(e),
    }
}

pub async fn select_page(
    sql: &str,
    param: &Value,
    page: u64,
    limit: u64,
) -> Result<PageResult<rbs::Value>> {
    let param = &mut check_param_for_query(param);
    param
        .as_object_mut()?
        .insert_string("_sql_type", "page_count".to_string())?;
    let count = select_value_to_primitive::<u64>(sql, param).await?;
    if count == 0 {
        return OK(PageResult {
            page,
            limit,
            total: count,
            list: vec![],
        });
    }
    let mut sql_new = sql.to_string();
    if !sql.contains_ignore_case(" offset ".to_string()) {
        let offset = get_offset(page, limit);
        if offset.is_err() {
            return Err(offset.err().unwrap());
        }
        sql_new = format!("{} offset {} limit {}", sql, offset.unwrap(), limit);
    };
    param
        .as_object_mut()?
        .insert_string("_sql_type", "page_list".to_string())?;
    let list = select_all(sql_new.as_str(), param).await?;
    OK(PageResult {
        page,
        limit,
        total: count,
        list,
    })
}

pub async fn select_page_to_entity<T>(
    sql: &str,
    param: &Value,
    page: u64,
    limit: u64,
) -> Result<PageResult<T>>
where
    T: for<'de> Deserialize<'de>,
{
    select_page(sql, param, page, limit)
        .await
        .map(|x| x.map(|x2| rbs::from_value::<T>(x2.clone()).unwrap()))
}

fn check_param_for_query(param: &Value) -> Value {
    if param.is_object() {
        param.clone()
    } else {
        let mut res = BTreeMap::<String, Value>::new();
        res.insert_value("_root", param.clone()).unwrap();
        Value::Object(res)
    }
}

pub async fn insert_returning_id(sql: &str, param: &Value) -> Result<rbs::Value> {
    let mut sql_new = sql.to_string();
    if !sql.contains_ignore_case("returning id".to_string()) {
        sql_new = format!("{} returning id", sql);
    };
    select_value(sql_new.as_str(), param).await
}

pub async fn execute_returning_row(sql: &str, param: &Value) -> Result<rbs::Value> {
    select_row(sql, param).await.map(|x| x.unwrap())
}

pub async fn execute_returning_value(sql: &str, param: &Value) -> Result<rbs::Value> {
    select_value(sql, param).await
}

pub async fn execute(sql: &str, param: &Value) -> Result<ExecResult> {
    let param = &check_param_for_query(param);
    let sql = render_sql_template(sql.to_string(), param)?;
    get_db()
        .await
        .exec(
            sql.0.as_str(),
            sql.1
                .iter()
                .map(|x| rbs::Value::from_value(x).unwrap())
                .collect::<Vec<rbs::Value>>(),
        )
        .await
        .map_err(|e| e.into())
}
