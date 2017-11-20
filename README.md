# json_config
json_config is a JSON based configuration management solution for Rust applications. It allows you to do the following,

1. Maintain application settings in JSON form.
2. Span your application settings across multiple different sources. i.e. From a file, as a string based variable, in a hard coded string literal and in pure JSON form within the code.
3. Maintain a base configuration and merge or override it with other JSON based configuration sections.
4. Define configuration bundles which encapsulate logically related configuration sections and override the base configuration when needed. i.e. Environment specific (QA, PROD, etc.) configuration bundles
5. Do all of the above either at runtime or at compile time via `build.rs`

The library also exposes macros which help you do all of the above in a very convenient manner. 

# Defining Your Configuration
Let's say that you have the following application settings to deal with,
```
{
    "appName": "json_config Demo",
    "appVersion": 1.5,
    "database": {
        "host": "dev.database.com",
        "port": 3000
    },
    "keystore": {
        "aws": "dev-aws-api-key",
        "googleMaps": "dev-google-api-key"
    },
    "translations": {
        "T001": "Welcome",
        "T002": "Thank you",
        "T003": "Have a nice day"
    }
}
```

Assume that you need these settings stored in different ways. Consider this scenario where;

The following config section is sourced from a string variable,
```
{
    "appName": "json_config Demo",
    "appVersion": 1.5
}
```

The following config section is hardcoded in the code,
```
{
    "database": {
        "host": "dev.database.com",
        "port": 3000
    }
}
```

The keystore and translation settings live in their own files in a folder titled "config" relative to the executable.

**config/keystore.json**
```
{    
    "keystore": {
        "aws": "dev-aws-api-key",
        "googleMaps": "dev-google-api-key"
    }
}
```

**config/translations.json**
```
{
    "translations": {
        "T001": "Welcome",
        "T002": "Thank you",
        "T003": "Have a nice day"
    }
}
```

# Building Your Configuration at Runtime
You can define and build your configuration during runtime and deserialize it to a predefined struct to get a type safe configuration object for use anywhere within the application like this,
```
#[macro_use] 
extern crate serde_derive; 
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate json_config;

use json_config::ConfigurationBuilder;
use json_config::ConfigurationSource;
use json_config::ConfigurationDefinitionParams;
use std::collections::HashMap;

//Define structs to hold the configuration in a type safe manner
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Configuration {
    app_name: String,
    app_version: f64,
    database: Database, 
    keystore: Keystore,
    translations: HashMap<String, String> //This can also be another struct instead of a HashMap
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Database {
    host: String,
    port: i32
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Keystore {
    aws: String,
    google_maps: String
}

fn main(){
        
    let app_config_str = r#"
    {
        "appName": "json_config Demo",
        "appVersion": 1.5
    }"#;

    //The config! macro returns a json_config::ConfigurationBuilder object
    //which contains methods for manipulating the imported configuration further
    let mut builder = config!(vec![ 
        //Sourced from a string variable       
        from_str!(app_config_str),
        
        //Hardcoded in pure JSON form
        from_json!({
            "database": {
                "host": "dev.database.com",
                "port": 3000
            }
        }),

        //Sourced from file
        from_file!("config/translations.json"),
        from_file!("config/keystore.json")        
    ]);

    //Pretty prints the merged configuration as a formatted JSON string
    println!("This how the merged configuration looks like in JSON form:");
    println!("{}", builder.to_string_pretty());

    //Get a typesafe configuration object and print the same 
    let config: Configuration = builder.to_type();
    println!("Your type safe configuration (same as above):");
    println!("{:?}", config);

    //Use the config object anywhere in your application 
    //to quickly access your configuration in a type safe manner
}
```

# Using the Configuration
The `config!` macro returns a `json_config::ConfigurationBuilder` object which can be used to further extend or override defined configuration. You could use the `to_type()` or the `to_enum()` methods of the `ConfigurationBuilder` object to obtain either a typed struct (as illustrated above) or a `serde_json::Value` enum which represents the final configuration for use within the application.

For further information on how to use the `serde_json::Value` enum, please refer the [serde_json documentation](https://docs.serde.rs/serde_json/value/index.html) 

# Predefined Configuration Bundles
Assume that you have two environments "QA" and "Production" to which you are expected to be deploying your application in addition to the development environment. The database and keystore settings are different for each environment. The latter resides in two different files like so,

**config/keystore_qa.json** (QA environment specific)
```
{
    "keystore": {
        "googleMaps": "qa-google-api-key",
        "aws": "qa-aws-api-key"
    }
}
```

**config/keystore_prod.json** (PROD environment specific)
```
{
    "keystore": {
        "googleMaps": "prod-google-api-key",
        "aws": "prod-aws-api-key"
    }
}
```

To address this you can define two configuration bundles, one for each environment which will be used to hold only the environment specific settings like so,

```
fn main(){
        
    let app_config_str = r#"
    {
        "appName": "json_config Demo",
        "appVersion": 1.5
    }"#;

    let mut builder = config!(vec![         
        from_str!(app_config_str),
        from_json!({
            "database": {
                "host": "dev.database.com",
                "port": 3000
            }
        }),
        from_file!("config/translations.json"),
        from_file!("config/keystore.json"),

        //QA Environment specific Settings
        bundle!("QA", vec![
            from_json!({
                "database": {
                    "host": "qa.database.com",
                    "port": 3001
                }
            }),
            from_file!("config/keystore_qa.json")  
        ]),

        //Production Environment specific Settings        
        bundle!("PROD",vec![
            from_json!({
                "database": {
                    "host": "prod.database.com",
                    "port": 3002
                }
            }),
            from_file!("config/keystore_prod.json") 
        ])

    ]);

    //Tell the builder to merge the PROD bundle and
    //override the base configuration settings
    builder.merge_bundle(&"PROD");

    println!("{}", builder.to_string_pretty());
}
```
As you may have already noticed, bundles are not automatically merged. Instead you need to tell the builder to merge one explicitly. Merging a bundle or a configuration source will always override if the same was defined previously. i.e. in this case, the database and keystore settings are overridden.

Here is the final output that you will see. Note that the database and keystore config sections now show the production environment specific settings

```
{
    "appName": "json_config Demo",
    "appVersion": 1.5,
    "database": {
        "host": "prod.database.com",
        "port": 3002
    },
    "keystore": {
        "aws": "prod-aws-api-key",
        "googleMaps": "prod-google-api-key"
    },
    "translations": {
        "T001": "Welcome",
        "T002": "Thank you",
        "T003": "Have a nice day"
    }
}
``` 

# Building Your Configuration at Compile-time
You probably may not want to build your configuration at runtime. It would make more sense to build it when the app compiles. To do this, you will need to do the same as above inside a build script (build.rs) and include the generated configuration in the main.rs at compile time. 

This helps you not only maintain a lean configuration in the binary, but also validate it at compile time.

Assume that the target environment bundle name (QA/PROD) is specified via the environment variable `JSON_CONFIG_ENV` before build time, so you can do `export JSON_CONFIG_ENV=PROD && cargo build`

**build.rs**
```
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate json_config;

use json_config::ConfigurationBuilder;
use json_config::ConfigurationSource;
use json_config::ConfigurationDefinitionParams;

fn main(){
        
    let app_config_str = r#"
    {
        "appName": "json_config Demo",
        "appVersion": 1.5
    }"#;

    let mut builder = config!(vec![         
        from_str!(app_config_str),
        from_json!({
            "database": {
                "host": "dev.database.com",
                "port": 3000
            }
        }),
        from_file!("config/translations.json"),
        from_file!("config/keystore.json"),

        //QA Environment specific Settings
        bundle!("QA", vec![
            from_json!({
                "database": {
                    "host": "qa.database.com",
                    "port": 3001
                }
            }),
            from_file!("config/keystore_qa.json")  
        ]),

        //Production Environment specific Settings        
        bundle!("PROD",vec![
            from_json!({
                "database": {
                    "host": "prod.database.com",
                    "port": 3002
                }
            }),
            from_file!("config/keystore_prod.json") 
        ])

    ]);

    //Tell the builder to merge the bundle specified
    //in the JSON_CONFIG_ENV environment variable
    builder.merge_bundle(env!("JSON_CONFIG_ENV"));

    //Outputs the built configuration to the file json_config.json 
    //alongside the binary in the output directory ("OUT_DIR")
    builder.to_compiled("json_config.json");
}
```

You guessed right! Bundles can have their own sources and merge order defined inside them too as illustrated above.

**main.rs**
```
#[macro_use]
extern crate json_config;

use json_config::ConfigurationBuilder;
use json_config::ConfigurationSource;
use json_config::ConfigurationDefinitionParams;

fn main(){   
    
    let builder = config!(vec![        
        //includes the generated json_config.json file content as a
        //string literal in the code at compile time
        from_compiled!("json_config.json")
    ]);
}
```

The `from_compiled!` macro can be used similarly to the other `from_str!` or `from_file!` macros. It will include the generated `json_config.json` file content as a string literal in the code at compile time.

# FAQ

### What if I want to package my settings as a separate file alongside the binary?
You could alternatively use `from_file!("json_config.json")` in place of `from_compiled!("json_config.json")` to make the `ConfigurationBuilder` import the file at runtime instead.

### Can I build the configuration at compile time and extend it at runtime?
Yes! You can have your cake and eat it too! There's nothing stopping you from extending the configuration further by merging more configuration sections at runtime. For instance, you may want to retrieve new translations via a service call and override the translations section based on a user's language selection at runtime like so,
```
#[macro_use]
extern crate json_config;

use json_config::ConfigurationBuilder;
use json_config::ConfigurationSource;
use json_config::ConfigurationDefinitionParams;

fn main(){
    //emulates retrieving a partial configuration via 
    //a remote API (i.e. REST) in json form
    let french_translations: String = get_translations("fr");
    
    let builder = config!(vec![                
        from_compiled!("json_config.json"),

        //overrides the translations
        from_str!(french_translations)
    ]);

    println!("{}", builder.to_string_pretty());
}

fn get_translations(_lang: &str) -> String{
    return String::from(r#"{ 
        "translations": { 
            "T001": "Bienvenue",
            "T002": "Je vous remercie",
            "T003": "Bonne journée"
        }
    }"#);
}
```

This will print the following output,
```
{
  "appName": "json_config Demo",
  "appVersion": 1.5,
  "database": {
    "host": "prod.database.com",
    "port": 3002
  },
  "keystore": {
    "aws": "prod-aws-api-key",
    "googleMaps": "prod-google-api-key"
  },
  "translations": {
    "T001": "Bienvenue",
    "T002": "Je vous remercie",
    "T003": "Bonne journée"
  }
}
```

### Do I have to use macros?
Not really. They are just there for convenience. You can alternatively use the methods found in the `ConfigurationBuilder` struct to achieve the desired effect. i.e. 

```
//Creating a ConfigurationBuilder instance from a source
let builder = ConfigurationBuilder::new(ConfigurationSource::FileContent("translations.json"));
```

```
//Merging a configuration source
builder.merge_source(&ConfigurationSource::StringContent(french_translations));
```

Refer the crate's documentation for more insight on the `ConfigurationBuilder`'s individual methods. (Work in progress...)

# License
Dual licensed under MIT/Apache-2.0.