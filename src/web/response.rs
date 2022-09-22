use knife_util::{
    bean::FromValueTrait,
    crates::{
        async_trait::async_trait,
        hyper::{Body, Response},
    },
    crates_builtin::serde_json,
    error::AppError,
    future::AsyncFrom,
    Result, Value, OK,
};
use tracing::info;

/// 响应对象
///
/// 现fn回调的结果会自动转换为hyper的response进行处理
pub struct HyperResponse {
    /// 响应结果
    resp: Option<Response<Body>>,
    /// 响应异常
    err: Option<AppError>,
}

#[async_trait]
impl AsyncFrom<(Result<Response<Body>>, String)> for HyperResponse {
    async fn async_from(res: (Result<Response<Body>>, String)) -> Self {
        match res.0 {
            Ok(v) => {
                info!("返回Web响应[{}],数据:$binary_data", res.1);
                HyperResponse {
                    resp: Some(v),
                    err: None,
                }
            }
            Err(e) => {
                info!("返回Web异常[{}],数据:{}", res.1, &e);
                HyperResponse {
                    resp: None,
                    err: Some(e),
                }
            }
        }
    }
}

#[async_trait]
impl AsyncFrom<(Result<Value>, String)> for HyperResponse {
    async fn async_from(res: (Result<Value>, String)) -> Self {
        match res.0 {
            Ok(v) => {
                let str = &serde_json::Value::from_value(&v).unwrap();
                let msg = serde_json::to_string(str).unwrap();
                info!("返回Web响应[{}],数据:{}", res.1, &msg);
                HyperResponse {
                    resp: Some(Response::new(Body::from(msg))),
                    err: None,
                }
            }
            Err(e) => {
                info!("返回Web异常[{}],数据:{}", res.1, &e);
                HyperResponse {
                    resp: None,
                    err: Some(e),
                }
            }
        }
    }
}

impl From<HyperResponse> for Result<Response<Body>> {
    fn from(v: HyperResponse) -> Self {
        if v.resp.is_some() {
            OK(v.resp.unwrap())
        } else {
            Err(v.err.unwrap())
        }
    }
}
