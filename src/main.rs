extern crate json_config;

use json_config::ConfigurationBuilder;
use json_config::ConfigurationSource;

fn main(){
    
    let mut builder = ConfigurationBuilder::new(&ConfigurationSource::StringContent(String::from(r#"{"test0": "val0"}"#)));

    builder.merge_source(&ConfigurationSource::StringContent(String::from(r#"{"test1": 1}"#)));

    let config_sources = vec![
        ConfigurationSource::StringContent(String::from(r#"{"test2": 1.234, "nested": { "nested1": "nestedValue1" }}"#)),
        ConfigurationSource::StringContent(String::from(r#"{"test3": true}"#))
        ];

    builder.merge_sources(&config_sources);  

    //println!("{}", builder.to_string_pretty());
    println!("{}", builder.to_enum().to_string());
}

// fn buildrs(){
//     let builder = ConfigurationBuilder::from_env("JSON_CONFIG_ENV");
//     builder.to_out_file("json_config.json");
// }

// fn main(){
//     let builder = from_out_file!("json_config.json");
//     println!("{}", builder.to_enum().to_string());
// }