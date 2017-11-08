extern crate serde;
extern crate serde_json;

use serde_json::Value;
use serde_json::from_str;

use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

pub enum ConfigSource {
    StringContent(String),
    FileContent(PathBuf),    
}

pub fn merge_configs(config_overrides: &Vec<ConfigSource>) -> Value{
    let mut merged_config: Value = from_str("{}").unwrap();
    
    for config in config_overrides{
        match config {
            &ConfigSource::StringContent(ref content) => {
                println!("{}", &content);
                
                let config_override: Value = from_str(&content[..]).unwrap();
                merge(&mut merged_config, config_override);
                //merge(&mut config, &json_override);
            },
            &ConfigSource::FileContent(ref path) => {
                let mut config_file = File::open(path).unwrap();
                let mut config_file_content = String::new();
                config_file.read_to_string(&mut config_file_content).unwrap();
                
                let config_override: Value = from_str(&config_file_content[..]).unwrap();
                merge(&mut merged_config, config_override);
            }
        }
    } 

    return merged_config;
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
