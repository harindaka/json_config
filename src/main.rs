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

//https://stackoverflow.com/questions/32956050/how-to-create-a-static-string-at-compile-time