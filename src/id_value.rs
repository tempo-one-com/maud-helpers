pub trait IdValue {
    fn id(&self) -> String;
    fn value(&self) -> String;
}

pub trait KVBuilderInterface {
    fn get_kv(&self) -> KeyValue;
    fn get_kv_box(&self) -> Box<KeyValue> {
        Box::new(self.get_kv())
    }
}

#[derive(Clone, Debug)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl IdValue for KeyValue {
    fn id(&self) -> String {
        self.key.clone()
    }

    fn value(&self) -> String {
        self.value.clone()
    }
}
