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

pub fn merge_configs(config_overrides: &Vec<ConfigSource>){
    for config in config_overrides{
        match config {
            &ConfigSource::StringContent(ref content) => {
                //let mut config: Value = from_str(include_str!("config/config.json")).unwrap();
                //merge(&mut config, &config_qa);
                //merge(&mut config, config_qa);
                println!("{}", content);
            },
            &ConfigSource::FileContent(ref path) => {
                let mut config_file = File::open(path).unwrap();
                let mut contents = String::new();
                config_file.read_to_string(&mut contents).unwrap();
                println!("{}", contents);
            }
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
