use serde::{Serialize, Deserialize};

/// 配置模块，用于控制配置文件的格式
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Setting {
    /// knife.xxx： 总设置
    #[serde(default)]
    pub knife: KnifeProp,
}

/// 总设置
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct KnifeProp {
    /// knife.project-id: 工程名
    #[serde(default, alias = "project-id")]
    pub project_id: String,
    /// knife.application-id: 应用名
    #[serde(default, alias = "application-id")]
    pub application_id: String,
    /// knife.cluster-id: 集群名
    #[serde(default, alias = "cluster-id")]
    pub cluster_id: String,
    /// knife.env-id: 环境名
    #[serde(default, alias = "env-id")]
    pub env_id: String,
    /// knife.env-profiles: 环境配置，用于指定特定配置选项
    #[serde(default, alias = "env-profiles")]
    pub env_profiles: Vec<String>,

    /// knife.command.xxx: 命令模块配置
    #[serde(default)]
    pub command: CommandProp,
    /// knife.web-server.xxx: Web服务器模块配置
    #[serde(default, alias = "web-server")]
    pub web_server: WebServerProp,
    /// knife.db.xxx: 数据源配置
    #[serde(default)]
    pub db: DbProp,
}

/// 命令模块配置，(预留)
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CommandProp {
    /// knife.command.name: 命令
    #[serde(default)]
    pub name: String,
    /// knife.command.about: 命令描述
    #[serde(default)]
    pub about: String,
}

/// Web服务器模块配置
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct WebServerProp {
    /// knife.web-server.port: 端口
    #[serde(default)]
    pub port: u16,
}

/// 数据源配置
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DbProp {
    /// knife.db.database_url: 数据源地址
    #[serde(default, alias = "database-url")]
    pub database_url: String,
}
