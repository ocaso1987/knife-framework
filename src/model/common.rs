//! 公共模型
use serde::{Deserialize, Serialize};

/// 应用参数属性模型
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AppInfo {
    /// 项目名称
    #[serde(default)]
    pub project_id: String,

    /// 应用名称
    #[serde(default)]
    pub application_id: String,

    /// 集群名称
    #[serde(default)]
    pub cluster_id: String,

    /// 环境名称
    #[serde(default)]
    pub env_id: String,
}

/// 应用参数查询请求模型
#[derive(Deserialize, Debug, Clone, Default)]
pub struct AppInfoQuery {
    /// 项目名称
    #[serde(default)]
    pub project_id: Option<String>,

    /// 应用名称
    #[serde(default)]
    pub application_id: Option<String>,

    /// 集群名称
    #[serde(default)]
    pub cluster_id: Option<String>,

    /// 环境名称
    #[serde(default)]
    pub env_id: Option<String>,
}

/// 返回码
#[derive(Serialize, Debug, Clone, Default)]
pub struct RespStatus {
    /// 响应码
    #[serde(default)]
    pub code: &'static str,
    /// 响应消息
    #[serde(default)]
    pub msg: &'static str,
}
