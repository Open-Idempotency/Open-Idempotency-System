

pub trait IConfig {
    fn get(&mut self, key: String) -> String;
}


