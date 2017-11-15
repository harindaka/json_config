extern crate serde_json;

mod macros;

use std::io::Read;
use std::collections::HashMap;
use std::{env};
use std::path::Path;
use std::io::{Write, BufWriter};
use std::fs::File;

use serde_json::Value;
use serde_json::from_str;
use serde_json::to_string_pretty;

#[derive(Clone)]
pub enum ConfigurationSource {
    StringContent(String),
    FileContent(String),
    JsonContent(Value)
}

pub enum ConfigurationDefinitionParams{
    Source(ConfigurationSource),
    Bundle(String, Vec<ConfigurationSource>)
}

pub struct ConfigurationBuilder {
    config: Value,
    bundles: HashMap<String, Vec<ConfigurationSource>>    
}

impl<'a> ConfigurationBuilder{

    pub fn new(base_source: ConfigurationSource) -> ConfigurationBuilder{
        let base_config: Value = from_str("{}").unwrap();
        
        let mut config_builder = ConfigurationBuilder{
            config: base_config,
            bundles: HashMap::new()
        };

        config_builder.merge_source(&base_source);

        return config_builder;
    }
    
    pub fn from_definition(definition: Vec<ConfigurationDefinitionParams>) -> ConfigurationBuilder{
        let mut builder = ConfigurationBuilder::new(ConfigurationSource::StringContent(String::from("{}")));

        for def_param in definition{
            match def_param{
                ConfigurationDefinitionParams::Source(source) => builder.merge_source(&source),
                ConfigurationDefinitionParams::Bundle(bundle_key, sources) => builder.define_bundle(bundle_key, sources)
            }
        }

        return builder;
    }

    pub fn merge_sources(&mut self, config_sources: &Vec<ConfigurationSource>){        
        for source in config_sources{
            self.merge_source(&source);
        }
    }

    pub fn merge_source(&mut self, config_source: &ConfigurationSource){            
        match config_source {
            &ConfigurationSource::JsonContent(ref config_override) => {                
                merge(&mut self.config, &config_override);
            },
            &ConfigurationSource::StringContent(ref content) => {                
                let config_override: Value = from_str(&content[..]).unwrap();
                merge(&mut self.config, &config_override);
            },
            &ConfigurationSource::FileContent(ref path) => {
                let mut config_file = File::open(path).unwrap();
                let mut config_file_content = String::new();
                config_file.read_to_string(&mut config_file_content).unwrap();
                
                let config_override: Value = from_str(&config_file_content[..]).unwrap();
                merge(&mut self.config, &config_override);
            }
        }      
    }

    pub fn define_bundle(&mut self, bundle_key: String, sources: Vec<ConfigurationSource>){
        self.bundles.insert(bundle_key, sources);
    }

    pub fn merge_bundle(&mut self, bundle_key: &str){
        let sources = self.bundles.get(bundle_key).unwrap().clone();
        self.merge_sources(&sources);
    }

    pub fn to_compiled(&mut self, filename: &str){
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join(filename);
        let mut f = BufWriter::new(File::create(&dest_path).unwrap());

        write!(f, "{}", self.config.to_string().as_str()).unwrap();
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

fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
