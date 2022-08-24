use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Setting {
    #[serde(default)]
    pub knife: KnifeProp,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct KnifeProp {
    #[serde(default, alias = "project-id")]
    pub project_id: String,
    #[serde(default, alias = "application-id")]
    pub application_id: String,
    #[serde(default, alias = "cluster-id")]
    pub cluster_id: String,
    #[serde(default, alias = "env-id")]
    pub env_id: String,
    #[serde(default, alias = "env-profiles")]
    pub env_profiles: Vec<String>,

    #[serde(default)]
    pub command: CommandProp,
    #[serde(default, alias = "web-server")]
    pub web_server: WebServerProp,
    #[serde(default)]
    pub db: DbProp,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CommandProp {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub about: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct WebServerProp {
    #[serde(default)]
    pub port: u16,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DbProp {
    #[serde(default)]
    pub driver_url: String,
}
