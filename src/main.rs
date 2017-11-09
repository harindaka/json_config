extern crate json_config;

use json_config::ConfigurationBuilder;
use json_config::ConfigurationSource;

fn main(){
    
    let mut builder = ConfigurationBuilder::new(&ConfigurationSource::StringContent(String::from(r#"{"test0": "val0"}"#)));

    builder.merge_source(&ConfigurationSource::StringContent(String::from(r#"{"test1": "val1"}"#)));

    let config_sources = vec![
        ConfigurationSource::StringContent(String::from(r#"{"test2": "val2"}"#)),
        ConfigurationSource::StringContent(String::from(r#"{"test3": "val3"}"#))
        ];

    builder.merge_sources(&config_sources);    

    println!("{}", builder.to_string_pretty());
}