use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct PageRequest {
    /// 页码
    #[serde(default)]
    pub page: u64,
    /// 每页条数
    #[serde(default)]
    pub limit: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct PageResult {
    /// 页码
    #[serde(default)]
    pub page: u64,
    /// 每页条数
    #[serde(default)]
    pub limit: u64,
    /// 总数
    #[serde(default)]
    pub total: u64,
}

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

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
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
