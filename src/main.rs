extern crate json_config;

use json_config::Configuration;
use json_config::ConfigSource;

fn main(){

    let config_overrides = vec![
        ConfigSource::StringContent(String::from(r#"{"test1": "val1"}"#)),
        ConfigSource::StringContent(String::from(r#"{"test2": "val2"}"#))
        ];

    let config = Configuration::new(&config_overrides);
    println!("{}", config.to_string());
}