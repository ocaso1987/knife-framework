use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct Setting {
    #[serde(default)]
    pub knife: KnifeProp,
}

impl Setting {
    pub fn merge(&self, target: Self) -> Self {
        let mut new_data = Self::default();
        new_data.knife = self.knife.merge(target.knife);
        new_data
    }
}

#[derive(Deserialize, Default, Debug)]
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

impl KnifeProp {
    fn merge(&self, target: Self) -> Self {
        let mut new_data = Self::default();
        new_data.project_id = if target.project_id.is_empty() {
            self.project_id.clone()
        } else {
            target.project_id
        };
        new_data.application_id = if target.application_id.is_empty() {
            self.application_id.clone()
        } else {
            target.application_id
        };
        new_data.cluster_id = if target.cluster_id.is_empty() {
            self.cluster_id.clone()
        } else {
            target.cluster_id
        };
        new_data.env_id = if target.env_id.is_empty() {
            self.env_id.clone()
        } else {
            target.env_id
        };
        new_data.env_profiles = if target.env_profiles.is_empty() {
            self.env_profiles.clone()
        } else {
            target.env_profiles
        };

        new_data.command = self.command.merge(target.command);
        new_data.web_server = self.web_server.merge(target.web_server);
        new_data.db = self.db.merge(target.db);
        new_data
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct CommandProp {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub about: String,
}

impl CommandProp {
    fn merge(&self, target: Self) -> Self {
        let mut new_data = Self::default();
        new_data.name = if target.name.is_empty() {
            self.name.clone()
        } else {
            target.name
        };
        new_data.about = if target.about.is_empty() {
            self.about.clone()
        } else {
            target.about
        };
        new_data
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct WebServerProp {
    #[serde(default = "super::default::webserver_port")]
    pub port: u16,
}

impl WebServerProp {
    fn merge(&self, target: Self) -> Self {
        let mut new_data = Self::default();
        new_data.port = if new_data.port == 0 {
            target.port
        } else if target.port != 8080 {
            self.port.clone()
        } else {
            target.port
        };
        new_data
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct DbProp {
    #[serde(default)]
    pub driver_url: String,
}

impl DbProp {
    fn merge(&self, target: Self) -> Self {
        let mut new_data = Self::default();
        if !target.driver_url.is_empty() {
            new_data.driver_url = target.driver_url.clone();
        }
        new_data
    }
}
