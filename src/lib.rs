use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

pub enum ConfigSource {
    StringContent(String),
    FileContent(PathBuf),    
}

pub fn my_function(config_overrides: Vec<ConfigSource>){
    for config in config_overrides{
        match config {
            ConfigSource::StringContent(content) => {
                println!("{}", content);
            },
            ConfigSource::FileContent(path) => {
                let mut config_file = File::open(path).unwrap();
                let mut contents = String::new();
                config_file.read_to_string(&mut contents).unwrap();
                println!("{}", contents);
            }
        }
    }    
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
