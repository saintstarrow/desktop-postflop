#![allow(deprecated)]

use std::collections::HashMap;
use std::sync::RwLock;
use config::{Config, File, ConfigError};

const CONFIG_FILE: &str = "Settings.toml";

lazy_static::lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new({
        let mut settings = Config::default();
        settings.merge(File::with_name(CONFIG_FILE)).unwrap();

        settings
    });
}

pub  fn read_bool(key: &str)->bool{
    SETTINGS.write().unwrap().refresh().unwrap().get_bool(key).unwrap()
}

// Result<f64, ConfigError>
pub fn read_float(key: &str) -> f64{    
    SETTINGS.write().unwrap().refresh().unwrap().get_float(key).unwrap()
   /* 
   let write = SETTINGS.write();
   match write {
        Ok(config) => {
            match config.refresh().get_float(key) {
                Ok(v) => Ok(v.clone().try_into().unwrap()),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(ConfigError::Message("failed to read config".to_string())),
    }*/
}

fn show() {
    println!(
        " * Settings :: \n\x1b[31m{:?}\x1b[0m",
        SETTINGS
            .read()
            .unwrap()
            .clone()
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );
}
