pub trait KeyValueInterface {
    fn to_kv(&self) -> KeyValue;
    // fn to_kv_box(&self) -> Box<KeyValue> {
    //     Box::new(self.to_kv())
    // }
}

#[derive(Clone, Debug)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl KeyValue {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: ToString,
        V: ToString,
    {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    pub fn new_key<K>(key: K) -> Self
    where
        K: ToString,
    {
        Self::new(key, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_str() {
        let kv = KeyValue::new("1", "un");

        assert_eq!(kv.key, "1");
    }

    #[test]
    fn constructor_string() {
        let kv = KeyValue::new(String::from("1"), "un");

        assert_eq!(kv.key, "1");
    }

    #[test]
    fn constructor_int() {
        let kv = KeyValue::new(1, "un");

        assert_eq!(kv.key, "1");
    }
}
