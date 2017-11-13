extern crate serde;
extern crate serde_json;
extern crate build_script_file_gen;

use serde_json::Value;
use serde_json::from_str;
use serde_json::to_string_pretty;
use build_script_file_gen::gen_file_str;

use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub enum ConfigurationSource {
    StringContent(String),
    FileContent(PathBuf)    
}

pub struct ConfigurationBuilder<'a> {
    config: Value,
    bundles: HashMap<&'a str, &'a Vec<ConfigurationSource>>    
}

impl<'a> ConfigurationBuilder<'a>{

    pub fn new(base_source: ConfigurationSource) -> ConfigurationBuilder<'a>{
        let base_config: Value = from_str("{}").unwrap();
        
        let mut config_builder = ConfigurationBuilder{
            config: base_config,
            bundles: HashMap::new()
        };

        config_builder.merge_source(&base_source);

        return config_builder;
    }

    pub fn merge_sources(&mut self, config_sources: &Vec<ConfigurationSource>){        
        for source in config_sources{
            self.merge_source(&source);
        }
    }

    pub fn merge_source(&mut self, config_source: &ConfigurationSource){            
        match config_source {
            &ConfigurationSource::StringContent(ref content) => {                
                let config_override: Value = from_str(&content[..]).unwrap();
                merge(&mut self.config, config_override);
                //merge(&mut config, &json_override);
            },
            &ConfigurationSource::FileContent(ref path) => {
                let mut config_file = File::open(path).unwrap();
                let mut config_file_content = String::new();
                config_file.read_to_string(&mut config_file_content).unwrap();
                
                let config_override: Value = from_str(&config_file_content[..]).unwrap();
                merge(&mut self.config, config_override);
            }
        }      
    }

    pub fn define_bundle(&mut self, bundle_key: &'a str, sources: &'a Vec<ConfigurationSource>){
        self.bundles.insert(bundle_key, sources);
    }

    pub fn merge_bundle(&mut self, bundle_key: &str){
        let sources = self.bundles[bundle_key];
        self.merge_sources(&sources);
    }

    pub fn to_compiled(&mut self, filename: &str){
        gen_file_str(filename, self.config.to_string().as_str());
    }

    pub fn to_string(&self) -> String{
        return self.config.to_string();
    }

    pub fn to_string_pretty(&self) -> String{
        return to_string_pretty(&self.config).unwrap();
    }

    pub fn to_enum(&self) -> Value{
        return self.config.clone();
    }
}

#[macro_export]
macro_rules! from_compiled {  
    ($file:expr) => {         
        ConfigurationSource::StringContent(String::from(include_file_str!($file)))
    }
}

#[macro_export]
macro_rules! from_str {  
    ($json:expr) => {         
        ConfigurationSource::StringContent($json)
    }
}

#[macro_export]
macro_rules! from_json {  
    ($json:expr) => {         
        ConfigurationSource::StringContent(String::from(r#"$json"#))
    }
}

#[macro_export]
macro_rules! from_file {  
    ($file_path:expr) => {                 
        ConfigurationSource::FileContent(String::from($file_path))
    }
}

// fn merge(a: &mut Value, b: &Value) {
//     match (a, b) {
//         (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
//             for (k, v) in b {
//                 merge(a.entry(k.clone()).or_insert(Value::Null), v);
//             }
//         }
//         (a, b) => {
//             *a = b.clone();
//         }
//     }
// }

fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
