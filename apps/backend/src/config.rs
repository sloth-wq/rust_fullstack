use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    database_url: String,
}

impl Config {
    pub fn database_url(&self) -> &String {
        &self.database_url
    }
}

pub fn get_config() -> Config {
    match envy::from_env::<Config>() {
        Ok(val) => val,
        Err(err) => {
            panic!("{}", err)
        }
    }
}
