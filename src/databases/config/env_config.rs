use std::env;
use log::{error, info};
use crate::databases::config::IConfig;

trait EnvConfig {}

impl IConfig for Box<dyn EnvConfig> {
    fn get(&mut self, key: String) -> String{
        match env::var_os(key) {
            Some(val) =>{
                info!("Key: {}, Value: {}", key, val.into_string()?);
                return val.into_string().unwrap();
            }
            None => {
                error!("{key} is not defined in the environment.");
                panic!("{key} does not exist")
            }
        }
    }
}
