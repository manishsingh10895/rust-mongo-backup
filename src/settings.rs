use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "./config/Default.toml";
const CONFIG_FILE_PREFIX: &str = "./config/";

#[derive(Debug, Clone, Deserialize)]
pub enum ENV {
    Development,
    Production,
}

impl std::fmt::Display for ENV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ENV::Development => write!(f, "Development"),
            ENV::Production => write!(f, "Production"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Production" => ENV::Production,
            _ => ENV::Development,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Root {
    pub user: String,
    pub pass: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Db {
    pub name: String,
    pub user: String,
    pub pass: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dbs {
    pub dbs: Vec<Db>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub root: Root,
    pub dbs: Dbs,
}

impl Settings {
    pub fn new(file: Option<std::path::PathBuf>) -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or(String::from("Default"));
        let mut s = Config::new();
        s.set("env", env.clone())?;

        match file {
            None => {
                s.merge(File::with_name(CONFIG_FILE_PATH))?;
                s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;

                s.merge(Environment::with_prefix("ea").separator("__"))?;
            }
            Some(val) => {
                let p = val.into_os_string().into_string().unwrap();
                println!("Config File, {}", p);
                s.merge(File::with_name(&p))?;
            }
        }

        s.try_into()
    }
}
