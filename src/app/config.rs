//! 配置加载模块
//! 
//! 用于从工程目录、系统目录或环境变量中加载配置信息
use std::{fs::File, io::Read, path::Path};

use knife_macro::knife_component;
use knife_util::{
    crates::{
        bson::{self, bson, Bson},
        serde_yaml, toml,
    },
    BsonConvertExt, MergeExt, Result, StringExt,
};
use tracing::{debug, info, trace};

use super::setting::Setting;

/// 配置数据结构体
#[knife_component(
    name = "GLOBAL_CONFIG",
    generate_method = "new",
    crate_builtin_name = "crate"
)]
pub struct Config {
    /// 被统一处理转换的默认数据格式
    pub setting: Option<Setting>,
    /// BSON存储的原始配置数据
    pub raw_setting: Option<Bson>,
    /// 程序启动通过注解配置的自定义配置数据
    pub custom_config: Vec<String>,
}

impl Config {
    /// 配置模块构造器
    pub(crate) fn new() -> Self {
        Config {
            setting: None,
            raw_setting: None,
            custom_config: vec![],
        }
    }

    /// 加载配置模块，并显示加载后数据概要信息
    pub(crate) async fn launch() -> Result<()> {
        let config = Config::get_instance() as &mut Config;
        config.load_from_env();
        config.load_from_file();
        config.set_default();

        let setting = app_setting();
        debug!("  - project_id: {}", setting.knife.project_id);
        debug!("  - application_id: {}", setting.knife.application_id);
        debug!("  - cluster_id: {}", setting.knife.cluster_id);
        debug!("  - env_id: {}", setting.knife.env_id);
        debug!("  - env_profiles: {:?}", setting.knife.env_profiles);
        trace!("\n---\n{}\n", serde_yaml::to_string(setting).unwrap());
        Ok(())
    }

    /// 为防止project_id和application_id被配置文件覆盖并进行强制锁定，设置配置空值时的默认参数
    pub fn set_default(&mut self) {
        let setting = self.setting.as_mut().unwrap();
        let mut origin_data = self.raw_setting.as_ref().unwrap().clone();
        let new_data = bson! ({
            "knife": {
                "project_id": std::env::var("knife_project_id").unwrap(),
                "application_id": std::env::var("knife_application_id").unwrap(),
                "cluster_id": setting.knife.cluster_id.if_blank("default".to_string()),
                "env_id": setting.knife.env_id.if_blank("default".to_string()),
                "command": {
                    "name": std::env::var("knife_command_name").unwrap_or("".to_string())
                }
            }
        });
        origin_data = origin_data.merge(&new_data).unwrap();
        self.setting
            .replace(bson::from_bson::<Setting>(origin_data.clone()).unwrap());
        self.raw_setting.replace(origin_data.clone());
    }

    /// 从配置文件中加载应用配置
    pub fn load_from_file(&mut self) {
        info!("从配置文件中加载应用配置...");
        let setting_all = self.setting_all();
        let setting_file = self.check_setting_file_persent(setting_all);
        let mut origin_data = self.raw_setting.as_ref().unwrap().clone();
        for path in setting_file {
            if path.ends_with(".yaml") || path.ends_with(".yml") {
                let str = &mut String::new();
                File::open(path).unwrap().read_to_string(str).unwrap();
                let new_data: serde_yaml::Value = serde_yaml::from_str(str).unwrap();
                origin_data = origin_data.merge(&new_data.as_bson()).unwrap();
            } else if path.ends_with(".toml") {
                let str = &mut String::new();
                File::open(path).unwrap().read_to_string(str).unwrap();
                let new_data: toml::Value = toml::from_str(str).unwrap();
                origin_data = origin_data.merge(&new_data.as_bson()).unwrap();
            }
        }
        for str in self.custom_config.iter() {
            let new_data: serde_yaml::Value = serde_yaml::from_str(str.as_str()).unwrap();
            origin_data = origin_data.merge(&new_data.as_bson()).unwrap();
        }
        self.setting
            .replace(bson::from_bson::<Setting>(origin_data.clone()).unwrap());
        self.raw_setting.replace(origin_data.clone());
    }

    /// 检查配置文件是否存在
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

    /// 拼装所有配置文件名称
    pub(crate) fn setting_all(&self) -> Vec<String> {
        let mut vec = Vec::<String>::new();
        let setting = self.setting.as_ref().unwrap();
        let env_id = setting.knife.env_id.as_str();
        let env_profiles = setting.knife.env_profiles.clone();
        for suffix in ["yaml", "yml", "toml"] {
            vec.push("application.".to_string() + suffix);
        }
        for suffix in ["yaml", "yml", "toml"] {
            vec.push("application-".to_string() + env_id + "." + suffix);
        }
        for profile in env_profiles {
            for suffix in ["yaml", "yml", "toml"] {
                vec.push("application_profile_".to_string() + profile.as_str() + "." + suffix);
            }
        }
        vec
    }

    /// 从上下文环境中加载应用配置
    pub fn load_from_env(&mut self) {
        info!("从上下文环境中加载应用配置...");
        let new_data = bson! ({
            "knife": {
                "project_id": std::env::var("knife_project_id").unwrap(),
                "application_id": std::env::var("knife_application_id").unwrap(),
                "cluster_id": std::env::var("knife_cluster_id").unwrap_or("".to_string()),
                "env_id": std::env::var("knife_env_id").unwrap_or("".to_string()),
                "command": {
                    "name": std::env::var("knife_command_name").unwrap_or("".to_string())
                }
            }
        });
        self.setting
            .replace(bson::from_bson::<Setting>(new_data.clone()).unwrap());
        self.raw_setting.replace(new_data.clone());
    }
}

/// 解析后的配置内容
pub fn app_setting() -> &'static Setting {
    let config = Config::get_instance() as &mut Config;
    config.setting.as_ref().unwrap()
}

/// BSON存储的原始配置内容
pub fn app_raw_setting() -> &'static Bson {
    let config = Config::get_instance() as &mut Config;
    config.raw_setting.as_ref().unwrap()
}

/// 添加自定义的Yaml格式配置
pub fn add_config(str: &'static str) {
    let config = Config::get_instance() as &mut Config;
    config.custom_config.push(str.to_string());
}
