use std::collections::BTreeMap;

use knife_util::{
    context::ContextExt,
    crates::rbatis::rbdc::db::ExecResult,
    error::ERR_DB_DATA,
    page::{get_offset, PageResult},
    template::render_sql_template,
    types::{StringExt, VecExt},
    value::{ConvertExt, Value},
    Ok, Result,
};
use serde::Deserialize;

use super::get_db;
use crate::crates::rbs;

pub async fn select_value(sql: String, param: &Value) -> Result<rbs::Value> {
    let v = select_row(sql, param).await.unwrap();
    match v {
        Some(v2) => {
            if v2.is_map() {
                let map = v2.as_map().unwrap();
                match map.first() {
                    Some((_k3, v3)) => Ok(v3.clone()),
                    None => Err(ERR_DB_DATA
                        .msg_detail("返回数据格式错误，期望单条数据，实际返回0条".to_string())),
                }
            } else if v2.is_array() {
                Err(ERR_DB_DATA
                    .msg_detail("返回数据格式错误，期望单条数据，实际返回多条".to_string()))
            } else {
                Ok(v2)
            }
        }
        None => {
            Err(ERR_DB_DATA.msg_detail("返回数据格式错误，期望单条数据，实际返回0条".to_string()))
        }
    }
}

pub async fn select_value_to_primitive<T>(sql: String, param: &Value) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    select_value(sql, param)
        .await
        .map(|x| rbs::from_value::<T>(x).unwrap())
}

pub async fn select_row(sql: String, param: &Value) -> Result<Option<rbs::Value>> {
    let param = &check_param_for_query(param);
    let sql = render_sql_template(sql.to_string(), param).unwrap();
    let res = get_db()
        .await
        .fetch(sql.0.as_str(), sql.1.map(|x| rbs::Value::from_value(x)))
        .await;
    match res {
        std::result::Result::Ok(v) => {
            if v.is_array() {
                let arr = v.as_array().unwrap();
                if arr.len() == 1 {
                    return Ok(Some(arr.get(0).unwrap().clone()));
                } else if arr.len() == 0 {
                    return Ok(None);
                } else {
                    return Err(ERR_DB_DATA.msg_detail("返回数据为多条".to_string()));
                }
            } else {
                return Err(ERR_DB_DATA.msg_detail("返回数据格式错误".to_string()));
            }
        }
        Err(e) => return Err(e.into()),
    }
}

pub async fn select_row_to_entity<T>(sql: String, param: &Value) -> Result<Option<T>>
where
    T: for<'de> Deserialize<'de>,
{
    select_row(sql, param)
        .await
        .map(|x| x.map(|x2| rbs::from_value::<T>(x2).unwrap()))
}

pub async fn select_all(sql: String, param: &Value) -> Result<Vec<rbs::Value>> {
    let param = &check_param_for_query(param);
    let sql = render_sql_template(sql.to_string(), param).unwrap();
    let res = get_db()
        .await
        .fetch(sql.0.as_str(), sql.1.map(|x| rbs::Value::from_value(x)))
        .await
        .map(|x| -> Vec<rbs::Value> {
            if x.is_array() {
                x.as_array().unwrap().map(|x2| x2.clone())
            } else {
                vec![x]
            }
        })
        .map_err(|e| e.into());
    res
}

pub async fn select_all_to_entity<T>(sql: String, param: &Value) -> Result<Vec<T>>
where
    T: for<'de> Deserialize<'de>,
{
    select_all(sql, param)
        .await
        .map(|x| x.map(|x2| rbs::from_value::<T>(x2.clone()).unwrap()))
}

pub async fn select_page(
    sql: String,
    param: &Value,
    page: u64,
    limit: u64,
) -> Result<PageResult<rbs::Value>> {
    let param = &mut check_param_for_query(param);
    param.with_object(|map| {
        map.insert_string("_sql_type", "page_count".to_string());
    });
    let count = select_value_to_primitive::<u64>(sql.to_string(), param)
        .await
        .unwrap();
    if count == 0 {
        return Ok(PageResult {
            page: page,
            limit: limit,
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
    param.with_object(|map| {
        map.insert_string("_sql_type", "page_list".to_string());
    });
    let list = select_all(sql_new.to_string(), param).await.unwrap();
    println!("list:{:?}", list);
    return Ok(PageResult {
        page: page,
        limit: limit,
        total: count,
        list: list,
    });
}

pub async fn select_page_to_entity<T>(
    sql: String,
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
        res.insert_value("_root", param.clone());
        Value::Object(res)
    }
}

pub async fn insert_returning_id(sql: String, param: &Value) -> Result<rbs::Value> {
    let mut sql_new = sql.to_string();
    if !sql.contains_ignore_case("returning id".to_string()) {
        sql_new = format!("{} returning id", sql);
    };
    select_value(sql_new, param).await
}

pub async fn execute_returning_row(sql: String, param: &Value) -> Result<rbs::Value> {
    select_row(sql, param).await.map(|x| x.unwrap())
}

pub async fn execute_returning_value(sql: String, param: &Value) -> Result<rbs::Value> {
    select_value(sql, param).await
}

pub async fn execute(sql: String, param: &Value) -> Result<ExecResult> {
    let param = &check_param_for_query(param);
    let sql = render_sql_template(sql.to_string(), param).unwrap();
    get_db()
        .await
        .exec(sql.0.as_str(), sql.1.map(|x| rbs::Value::from_value(x)))
        .await
        .map_err(|e| e.into())
}
