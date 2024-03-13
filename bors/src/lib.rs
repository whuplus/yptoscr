pub struct Serializable {
    pub key: String,
    pub value: String,
}

pub fn serialize(serializable: &Serializable, new_flag: bool) -> String {
    format!("{}={}", serializable.key, serializable.value)
}

pub fn deserialize(str: &String) -> Serializable {
    Serializable {
        key: str[0..str.find("=").unwrap()].to_string(),
        value: str[str.find("=").unwrap() + 1..].to_string(),
    }
}
