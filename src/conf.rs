use crate::error::GoukiError;
use std::collections::HashMap;
use std::fs;

const CONFIG_FILE: &str = "gouki.conf";

pub struct Conf {
    pub ip: String,
    pub port: u16,
}

pub fn get_conf_from_file() -> Result<Conf, GoukiError> {
    let mut conf_map = HashMap::new();
    let contents = fs::read_to_string(CONFIG_FILE).unwrap();
    for line in contents.lines() {
        let mut split = line.split("=");
        let (key, value) = (split.next().unwrap(), split.next().unwrap());
        if key.to_lowercase() == "ip" {
            conf_map.insert("ip", value);
        } else if key.to_lowercase() == "port" {
            conf_map.insert("port", value);
        }
    }
    Ok(Conf {
        ip: String::from(*conf_map.get("ip").unwrap()),
        port: (*conf_map.get("port").unwrap()).parse::<u16>().unwrap(),
    })
}
