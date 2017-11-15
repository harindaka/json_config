#[macro_use]
extern crate json_config;

use json_config::ConfigurationBuilder;
use json_config::ConfigurationSource;
use json_config::ConfigurationDefinitionParams;

fn main(){
    
    let mut builder = config!(vec![
        from_str!(r#"{
            "appName": "json_config Demo",
            "appVersion": "1.0",
            "database": {
                "host": "dev.database.com",
                "port": 3000
            }
        }"#),
        from_file!("config/translations.json"),
        from_file!("config/keystore.json"),

        bundle!("QA", vec![
            from_str!(r#"{
                "database": {
                    "host": "qa.database.com",
                    "port": 3001
                }
            }"#),
            from_file!("config/keystore_qa.json")  
        ]),

        bundle!("PROD",vec![
            from_str!(r#"{
                "database": {
                    "host": "prod.database.com",
                    "port": 3002
                }
            }"#),
            from_file!("config/keystore_prod.json") 
        ])
    ]);
   
    builder.merge_bundle(&"PROD");

    //emulates retrieving a partial configuration via 
    //a remote API (i.e. REST) in json form
    let remote_config: String = get_remote_config("fr");
    builder.merge_source(&ConfigurationSource::StringContent(remote_config));

    println!("{}", builder.to_string_pretty());
}

fn get_remote_config(lang: &str) -> String{    
    return String::from(r#"{ 
        "translations": { 
            "T001": "Bienvenue",
            "T002": "Je vous remercie",
            "T003": "Bonne journée"
        }
    }"#);
}

// // fn buildrs(){
// //     builder = config!([
// //         from_str!(r#"{
// //             "database": {
// //                 "host": "dev.database.com"
// //                 "port": 3000
// //             }
// //         }"#),
// //         from_file!("translations.json"),
// //         from_file!("api_keys.json"),

// //         bundle!("QA",[
// //             from_str!(r#"{
// //                 "database": {
// //                     "host": "qa.database.com"
// //                     "port": 3001
// //                 }
// //             }"#),
// //             from_file!("api_keys_qa.json")  
// //         ]),

// //         bundle!("PROD",[
// //             from_str!(r#"{
// //                 "database": {
// //                     "host": "prod.database.com"
// //                     "port": 3002
// //                 }
// //             }"#),
// //             from_file!("api_keys_prod.json") 
// //         ])
// //     ]);
// //
// //     builder.merge_bundle(env!("JSON_CONFIG_ENV"));
// //     builder.to_compiled("json_config.json");
// // }

// // fn main(){
// //     //emulates retrieving a partial configuration via 
// //     //a remote API (i.e. REST) in json form
// //     let remote_config: String: get_remote_config("fr");
    
// //     let builder = config!([
// //         from_compiled!("json_config.json"),
// //         from_str!(remote_config)
// //     ]);

// //     println!("{}", builder.to_string_pretty());
// // }

// // fn get_remote_config(lang: &str) -> String{    
// //     return String::from(r#"{ "translations": { 
// //         "T001": "Bienvenue",
// //         "T002": "Je vous remercie"
// //         "T003": "Bonne journée"
// //     }"#);

// //     // en_translations.json
// //     // {
// //     //     "T001": "Welcome",
// //     //     "T002": "Thank you",
// //     //     "T003": "Have a nice day"
// //     // }
// // }