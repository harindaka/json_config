#[macro_use]
extern crate json_config;

use json_config::ConfigurationBuilder;
use json_config::ConfigurationSource;

fn main(){
    
    // let base_config = ConfigurationSource::StringContent(String::from(r#"{"test0": "val0"}"#));
    // let mut builder = ConfigurationBuilder::new(base_config);

    let base_config_str = String::from(r#"{"fromString": "from_string"}"#);
    let base_config = from_str!(base_config_str);
    let mut builder = ConfigurationBuilder::new(base_config);

    builder.merge_source(&from_str!(r#"{"fromStr": "from_str"}"#));
    builder.merge_source(&from_file!("/home/harindaka/source/github/json_config/config/translations.json"));
    
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
//     builder.merge_bundle(env!("JSON_CONFIG_ENV")); //clone and lazy merge env
//     builder.to_compiled("json_config.json");

//     builder = config!([
//         from_json!(r#"{
//             "database": {
//                 "host": "dev.database.com"
//                 "port": 3000
//             }
//         }"#),
//         from_file!("translations.json"),
//         from_file!("api_keys.json"),

//         bundle!("QA",[
//             from_json!(r#"{
//                 "database": {
//                     "host": "qa.database.com"
//                     "port": 3001
//                 }
//             }"#),
//             from_file!("api_keys.json")  
//         ]),

//         bundle!("PROD",[
//             from_json!(r#"{
//                 "database": {
//                     "host": "prod.database.com"
//                     "port": 3002
//                 }
//             }"#),
//             from_file!("api_keys.json") 
//         ])
//     ]);
//
//     builder.merge_bundle(env!("JSON_CONFIG_ENV"));
//     builder.to_compiled("json_config.json");
// }

// fn main(){
//     //emulates retrieving a partial configuration via 
//     //a remote API (i.e. REST) in json form
//     let remote_config: String: get_remote_config("fr");
    
//     let builder = config!([
//         from_compiled!("json_config.json"),
//         from_string!(remote_config)
//     ]);

//     println!("{}", builder.to_string_pretty());
// }

// fn get_remote_config(lang: &str) -> String{    
//     return String::from(r#"{ "translations": { 
//         "T001": "Bienvenue",
//         "T002": "Je vous remercie"
//         "T003": "Bonne journée"
//     }"#);

//     // en_translations.json
//     // {
//     //     "T001": "Welcome",
//     //     "T002": "Thank you",
//     //     "T003": "Have a nice day"
//     // }
// }