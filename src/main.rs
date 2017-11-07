extern crate json_config;

use json_config::merge_configs;
use json_config::ConfigSource;

fn main(){

    let config_overrides = vec![
        ConfigSource::StringContent(String::from(r#"{test1: "val1"}"#)),
        ConfigSource::StringContent(String::from(r#"{test2: "val2"}"#))
        ];

    merge_configs(&config_overrides);
}