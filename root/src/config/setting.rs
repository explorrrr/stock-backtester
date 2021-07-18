use envy;
use serde::Deserialize;
use std::process;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_addr: String,
    pub database_url: String,
}

pub fn get_config() -> Config {
    // 設定項目の取得
    let config: Config = match envy::from_env::<Config>() {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    config
}
