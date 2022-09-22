//! 配置加载模块
//!
//! 用于从工程目录、系统目录或环境变量中加载配置信息
use std::{fs::File, io::Read, path::Path};

use knife_macro::knife_component;
use knife_util::{
    bean::{AsValueTrait, FromValueTrait, MergeTrait, PointerTrait},
    crates_builtin::serde_yaml,
    types::StringExt,
    Result, Value, OK,
};
use tracing::{debug, info, trace};

use crate::value;

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
    /// 内置Value存储的原始配置数据
    pub raw_setting: Option<Value>,
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
    pub(crate) async fn launch() {
        let config: &mut Config = Config::get_instance();
        config.load_from_env().unwrap();
        config.load_from_file().unwrap();
        config.set_default().unwrap();

        let setting = app_setting();
        debug!("  - project_id: {}", setting.knife.project_id);
        debug!("  - application_id: {}", setting.knife.application_id);
        debug!("  - cluster_id: {}", setting.knife.cluster_id);
        debug!("  - env_id: {}", setting.knife.env_id);
        debug!("  - env_profiles: {:?}", setting.knife.env_profiles);
        trace!("\n---\n{}\n", serde_yaml::to_string(setting).unwrap());
    }

    /// 为防止project_id和application_id被配置文件覆盖并进行强制锁定，设置配置空值时的默认参数
    pub fn set_default(&mut self) -> Result<()> {
        let raw_setting = self.raw_setting.as_ref().unwrap();
        let mut origin_data = self.raw_setting.as_ref().unwrap().clone();
        let new_data = value! ({
            "knife": {
                "project_id": std::env::var("knife_project_id")?,
                "application_id": std::env::var("knife_application_id")?,
                "cluster_id": raw_setting.p("/knife/cluster_id").unwrap().as_str()?.if_blank("default".to_string()),
                "env_id":  raw_setting.p("/knife/env_id").unwrap().as_str()?.if_blank("default".to_string()),
                "command": {
                    "name": std::env::var("knife_command_name").unwrap_or_else(|_| "".to_string())
                }
            }
        });
        origin_data = origin_data.merge_self(&new_data)?;
        self.raw_setting.replace(origin_data.clone());

        let setting =
            serde_yaml::from_value::<Setting>(serde_yaml::Value::from_value(&origin_data)?)?;
        self.setting.replace(setting);
        OK(())
    }

    /// 从配置文件中加载应用配置
    pub fn load_from_file(&mut self) -> Result<()> {
        info!("从配置文件中加载应用配置...");
        let setting_all = self.setting_all()?;
        let setting_file = self.check_setting_file_persent(setting_all)?;
        let mut origin_data = self.raw_setting.as_ref().unwrap().clone();
        for path in setting_file {
            if path.ends_with(".yaml") || path.ends_with(".yml") {
                let str = &mut String::new();
                File::open(path)?.read_to_string(str)?;
                let new_data: serde_yaml::Value = serde_yaml::from_str(str)?;
                origin_data = origin_data.merge_self(&new_data.as_value()?)?
            }
        }
        for str in self.custom_config.iter() {
            let new_data: serde_yaml::Value = serde_yaml::from_str(str.as_str())?;
            origin_data = origin_data.merge_self(&new_data.as_value()?)?;
        }
        self.raw_setting.replace(origin_data.clone());
        OK(())
    }

    /// 检查配置文件是否存在
    pub(crate) fn check_setting_file_persent(&self, setting: Vec<String>) -> Result<Vec<String>> {
        let mut vec = Vec::<String>::new();
        for filename in setting {
            let filename = "./config/".to_string() + &filename;
            let path = Path::new(&filename);
            if path.exists() {
                let path = path.canonicalize()?.to_str().unwrap().to_string();
                debug!("检测到配置文件:{:?}", path);
                vec.push(path)
            }
        }
        OK(vec)
    }

    /// 拼装所有配置文件名称
    pub(crate) fn setting_all(&self) -> Result<Vec<String>> {
        let mut vec = Vec::<String>::new();
        let raw_setting = self.raw_setting.as_ref().unwrap();
        let env_id = raw_setting.p("/knife/env_id").unwrap().as_str()?;
        let env_profiles = raw_setting
            .p("/knife/env_profiles")
            .unwrap()
            .as_array()?
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect::<Vec<String>>();
        for suffix in ["yaml", "yml"] {
            vec.push("application.".to_string() + suffix);
        }
        for suffix in ["yaml", "yml"] {
            vec.push("application-".to_string() + env_id + "." + suffix);
        }
        for profile in env_profiles {
            for suffix in ["yaml", "yml"] {
                vec.push("application_profile_".to_string() + profile.as_str() + "." + suffix);
            }
        }
        OK(vec)
    }

    /// 从上下文环境中加载应用配置
    pub fn load_from_env(&mut self) -> Result<()> {
        info!("从上下文环境中加载应用配置...");
        let new_data = value! ({
            "knife": {
                "project_id": std::env::var("knife_project_id")?,
                "application_id": std::env::var("knife_application_id")?,
                "cluster_id": std::env::var("knife_cluster_id").unwrap_or_else(|_| "".to_string()),
                "env_id": std::env::var("knife_env_id").unwrap_or_else(|_| "".to_string()),
                "env_profiles": std::env::var("knife_env_profiles").unwrap_or_else(|_| "".to_string()).split(',').collect::<Vec<&str>>(),
                "command": {
                    "name": std::env::var("knife_command_name").unwrap_or_else(|_| "".to_string())
                }
            }
        });
        self.raw_setting.replace(new_data);
        OK(())
    }
}

/// 解析后的配置内容
pub fn app_setting() -> &'static Setting {
    let config: &mut Config = Config::get_instance();
    config.setting.as_ref().unwrap()
}

/// 内置Value存储的原始配置内容
pub fn app_raw_setting() -> &'static Value {
    let config: &mut Config = Config::get_instance();
    config.raw_setting.as_ref().unwrap()
}

/// 添加自定义的Yaml格式配置
pub fn add_config(str: &'static str) {
    let config: &mut Config = Config::get_instance();
    config.custom_config.push(str.to_string());
}
