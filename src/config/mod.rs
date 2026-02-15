

use std::{collections::HashMap, fs::File, io::Read, sync::LazyLock};

use serde::{Serialize, Deserialize};



pub static SERVER_CONFIG: LazyLock<Config> = LazyLock::new (|| 
    {
        let config_file = File::open("config.json");
    
        let mut file_content = String::new();
    
        let _ = config_file.unwrap().read_to_string(&mut file_content);

        let out = serde_json::from_str(&file_content);

        out.unwrap()
    }

); 

#[derive(Serialize, Deserialize, Debug)]
pub struct ListenerConfig {
    #[serde( alias = "IP")]
    pub ip: String,


    #[serde(alias = "KIND")]
    pub kind: String,

    #[serde(alias = "PORT")]
    pub port: u16
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(alias = "LISTENERS")]

    pub listeners: HashMap<String, ListenerConfig>
}

impl Config {
    
}