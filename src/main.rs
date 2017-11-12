#[macro_use]
extern crate json_config;

use json_config::ConfigurationBuilder;
use json_config::ConfigurationSource;

fn main(){
    
    // let base_config = ConfigurationSource::StringContent(String::from(r#"{"test0": "val0"}"#));
    // let mut builder = ConfigurationBuilder::new(base_config);

    let mut builder = config_str!(r#"{"test0": "val0"}"#);
    
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
//     let mut builder = config_str!(r#"{"test0": "val0"}"#);
//     builder.define_bundle("qa", config_sources); //just store
//     builder.define_bundle("prod", config_sources); //just store
//     buider.merge_bundle(env!("JSON_CONFIG_ENV")); //clone and lazy merge env
//     builder.to_compiled("json_config.json");
// }

// fn main(){
//     let builder = config_compiled!("json_config.json");
//     println!("{}", builder.to_string_pretty());
// }