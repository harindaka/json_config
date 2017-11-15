#[macro_export]
macro_rules! from_compiled {  
    ($file:expr) => {         
        ConfigurationDefinitionParams::Source(ConfigurationSource::StringContent(String::from(include_file_str!($file))))
    }
}

#[macro_export]
macro_rules! from_str {  
    ($json:expr) => {         
        ConfigurationDefinitionParams::Source(ConfigurationSource::StringContent(String::from($json)))
    }
}

#[macro_export]
macro_rules! from_json {  
    ($($json:tt)+) => {         
        ConfigurationDefinitionParams::Source(ConfigurationSource::JsonContent(json!($($json)+)))
    }
}

#[macro_export]
macro_rules! from_file {  
    ($file_path:expr) => {                 
        ConfigurationDefinitionParams::Source(ConfigurationSource::FileContent(String::from($file_path)))
    }
}

#[macro_export]
macro_rules! bundle {  
    ($bundle_key:expr, $bundle_sources:expr) => {{       
        let mut config_sources = Vec::new();

        for bundle in $bundle_sources{
            match bundle {
                ConfigurationDefinitionParams::Source(config_source) => config_sources.push(config_source),
                ConfigurationDefinitionParams::Bundle(_,_) => panic!("Nested bundle! declarations are not supported."),
            }
        }
        
        ConfigurationDefinitionParams::Bundle(String::from($bundle_key), config_sources)
    }}
}

#[macro_export]
macro_rules! config {  
    ($definition:expr) => {
        ConfigurationBuilder::from_definition($definition);
    }
}