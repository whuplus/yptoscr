use bors::Serializable;

pub fn make_serializable(key: String, value: String) -> Serializable {
    Serializable { key, value }
}
