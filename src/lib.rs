extern crate serde;
extern crate serde_json;

mod macros;
mod error;

use std::io::Read;
use std::collections::HashMap;
use std::{env};
use std::path::Path;
use std::io::{Write, BufWriter};
use std::fs::File;

use serde_json::Value;
use serde_json::from_str;
use serde_json::to_string_pretty;
use serde_json::from_value;
use serde::de::DeserializeOwned;
use error::JsonConfigError;
//use serde_json::error::Error;

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

    pub fn new(base_source: ConfigurationSource) -> Result<ConfigurationBuilder, JsonConfigError>{
        let base_config: Value = try!(from_str("{}"));
        
        let mut config_builder = ConfigurationBuilder{
            config: base_config,
            bundles: HashMap::new()
        };

        try!(config_builder.merge_source(&base_source));

        return Ok(config_builder);
    }
    
    pub fn from_definition(definition: Vec<ConfigurationDefinitionParams>) -> Result<ConfigurationBuilder, JsonConfigError>{
        let mut builder = try!(ConfigurationBuilder::new(ConfigurationSource::StringContent(String::from("{}"))));

        for def_param in definition{
            match def_param{
                ConfigurationDefinitionParams::Source(source) => try!(builder.merge_source(&source)),
                ConfigurationDefinitionParams::Bundle(bundle_key, sources) => builder.define_bundle(bundle_key, sources)
            }
        }

        Ok(builder)
    }

    pub fn merge_sources(&mut self, config_sources: &Vec<ConfigurationSource>){        
        for source in config_sources{
            self.merge_source(&source);
        }
    }

    pub fn merge_source(&mut self, config_source: &ConfigurationSource) -> Result<(), JsonConfigError>{            
        match config_source {
            &ConfigurationSource::JsonContent(ref config_override) => {                
                merge(&mut self.config, &config_override);
            },
            &ConfigurationSource::StringContent(ref content) => {                
                let config_override: Value = try!(from_str(&content[..]));
                merge(&mut self.config, &config_override);
            },
            &ConfigurationSource::FileContent(ref path) => {
                let mut config_file = try!(File::open(path));
                let mut config_file_content = String::new();
                try!(config_file.read_to_string(&mut config_file_content));
                
                let config_override: Value = try!(from_str(&config_file_content[..]));
                merge(&mut self.config, &config_override);
            }
        }
        
        Ok(())    
    }

    pub fn define_bundle(&mut self, bundle_key: String, sources: Vec<ConfigurationSource>){
        self.bundles.insert(bundle_key, sources);
    }

    pub fn merge_bundle(&mut self, bundle_key: &str) -> Result<(), JsonConfigError>{
        let sources = match self.bundles.get(bundle_key){
            Some(bundle) => bundle.clone(),
            None => return Err(JsonConfigError::ConfigDefinition("".to_string())),
        };
        
        self.merge_sources(&sources);

        Ok(())
    }

    pub fn to_compiled(&mut self, filename: &str) -> Result<(), JsonConfigError>{
        let out_dir = try!(env::var("OUT_DIR"));
        let dest_path = Path::new(&out_dir).join(filename);
        let mut f = BufWriter::new(try!(File::create(&dest_path)));

        try!(write!(f, "{}", self.config.to_string().as_str()));

        Ok(())
    }

    pub fn to_string(&self) -> String{
        return self.config.to_string();
    }

    pub fn to_string_pretty(&self) -> Result<String, JsonConfigError>{
        Ok(try!(to_string_pretty(&self.config)))
    }

    pub fn to_enum(&self) -> Value{
        return self.config.clone();
    }

    pub fn to_type<T>(&self) -> Result<T, JsonConfigError>
    where
        T: DeserializeOwned,
    {
        Ok(try!(from_value(self.config.clone())))
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
