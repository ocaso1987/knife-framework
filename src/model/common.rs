use knife_util::VecExt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct PageRequest<T> {
    /// 页码
    #[serde(default)]
    pub page: u64,
    /// 每页条数
    #[serde(default)]
    pub limit: u64,
    #[serde(flatten, default)]
    pub target: T,
}

impl<T> PageRequest<T> {
    pub fn map<F, R>(&self, fun: F) -> PageRequest<R>
    where
        F: Fn(&T) -> R,
    {
        PageRequest {
            page: self.page,
            limit: self.limit,
            target: fun(&self.target),
        }
    }
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct PageResult<T> {
    /// 页码
    #[serde(default)]
    pub page: u64,
    /// 每页条数
    #[serde(default)]
    pub limit: u64,
    /// 总数
    #[serde(default)]
    pub total: u64,
    /// 返回列表
    #[serde(default)]
    pub list: Vec<T>,
}

impl<T> PageResult<T> {
    pub fn map<F, R>(&self, fun: F) -> PageResult<R>
    where
        F: Fn(&T) -> R,
    {
        PageResult {
            page: self.page,
            limit: self.limit,
            total: self.total,
            list: self.list.map(fun),
        }
    }
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

#[derive(Serialize, Debug, Clone, Default)]
pub struct RespStatus {
    /// 响应码
    #[serde(default)]
    pub code: &'static str,
    /// 响应消息
    #[serde(default)]
    pub msg: &'static str,
}
