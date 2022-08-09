use std::{fs::File, io::Read, path::Path};

use knife_macro::knife_component;
use knife_util::AnyValue;
use tracing::{debug, info};

use super::setting::Setting;

#[knife_component(
    name = "GLOBAL_CONFIG",
    generate_method = "new",
    crate_builtin_name = "crate"
)]
pub struct Config {
    pub setting: AnyValue,
    pub custom_config: Vec<String>,
}

impl Config {
    pub(crate) fn new() -> Self {
        Config {
            setting: AnyValue::new(Setting::default()),
            custom_config: vec![],
        }
    }
    pub(crate) async fn launch() {
        let config = Config::get_instance() as &mut Config;
        config.load_from_env();
        config.load_from_file();
        config.set_default();

        let setting = app_setting();
        debug!("  - project_id: {}", setting.knife.project_id);
        debug!("  - application_id: {}", setting.knife.application_id);
        debug!("  - cluster_id: {}", setting.knife.cluster_id);
        debug!("  - env_id: {}", setting.knife.env_id);
        debug!("  - env_profiles: {}", setting.knife.env_profiles.join(","));
    }

    pub fn set_default(&mut self) {
        let mut setting = self.setting.take::<Setting>();
        setting.knife.project_id = std::env::var("knife_project_id").unwrap();
        setting.knife.application_id = std::env::var("knife_application_id").unwrap();
        setting.knife.command.name = std::env::var("knife_command_name").unwrap_or("".to_string());
        if setting.knife.cluster_id.is_empty() {
            setting.knife.cluster_id = "local".to_string();
        }
        if setting.knife.env_id.is_empty() {
            setting.knife.env_id = "default".to_string();
        }
        self.setting = AnyValue::new(setting);
    }

    pub fn load_from_file(&mut self) {
        info!("从配置文件中加载应用配置...");
        let setting_all = self.setting_all();
        let setting_file = self.check_setting_file_persent(setting_all);
        let mut origin_data = self.setting.take::<Setting>();
        for path in setting_file {
            if path.ends_with(".yaml") || path.ends_with(".yml") {
                let str = &mut String::new();
                File::open(path).unwrap().read_to_string(str).unwrap();
                let new_data: Setting = knife_util::serde_yaml::from_str(str).unwrap();
                origin_data = origin_data.merge(new_data);
            } else if path.ends_with(".toml") {
                let str = &mut String::new();
                File::open(path).unwrap().read_to_string(str).unwrap();
                let new_data: Setting = knife_util::toml::from_str(str).unwrap();
                origin_data = origin_data.merge(new_data);
            }
        }
        for str in self.custom_config.iter() {
            let new_data: Setting = knife_util::serde_yaml::from_str(str.as_str()).unwrap();
            origin_data = origin_data.merge(new_data);
        }
        self.setting = AnyValue::new(origin_data);
    }

    pub(crate) fn check_setting_file_persent(&self, setting: Vec<String>) -> Vec<String> {
        let mut vec = Vec::<String>::new();
        for filename in setting {
            let filename = "./config/".to_string() + &filename;
            let path = Path::new(&filename);
            if path.exists() {
                let path = path.canonicalize().unwrap().to_str().unwrap().to_string();
                debug!("检测到配置文件:{:?}", path);
                vec.push(path)
            }
        }
        vec
    }

    pub(crate) fn setting_all(&self) -> Vec<String> {
        let mut vec = Vec::<String>::new();
        let knife_prop = &self.setting.as_ref::<Setting>().knife;
        for suffix in ["yaml", "yml", "toml"] {
            vec.push("application.".to_string() + suffix);
        }
        for suffix in ["yaml", "yml", "toml"] {
            vec.push("application_".to_string() + &knife_prop.env_id + "." + suffix);
        }
        for profile in &knife_prop.env_profiles {
            for suffix in ["yaml", "yml", "toml"] {
                vec.push("application_profile_".to_string() + profile + "." + suffix);
            }
        }
        vec
    }

    pub fn load_from_env(&mut self) {
        info!("从上下文环境中加载应用配置...");
        let mut origin_data = self.setting.take::<Setting>();
        let mut new_data = Setting::default();
        new_data.knife.project_id = std::env::var("knife_project_id").unwrap();
        new_data.knife.application_id = std::env::var("knife_application_id").unwrap();
        new_data.knife.command.name = std::env::var("knife_command_name").unwrap_or("".to_string());
        origin_data = origin_data.merge(new_data);
        self.setting = AnyValue::new(origin_data)
    }
}

pub fn app_setting() -> &'static Setting {
    let config = Config::get_instance() as &mut Config;
    config.setting.as_ref()
}

pub fn add_config(str: &'static str) {
    let config = Config::get_instance() as &mut Config;
    config.custom_config.push(str.to_string());
}
