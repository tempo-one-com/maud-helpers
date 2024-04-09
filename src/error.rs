use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ValidationErrors {
    default_value: String,
    errors: HashMap<String, String>,
}

impl ValidationErrors {
    pub fn new(default_value: &str) -> Self {
        Self {
            default_value: default_value.to_owned(),
            ..Default::default()
        }
    }

    pub fn set_default(&mut self, key: &str) {
        self.errors
            .insert(key.to_owned(), self.default_value.clone());
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.errors.insert(key.to_owned(), value.to_owned());
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.errors.get(key).cloned()
    }

    pub fn has(&self, key: &str) -> bool {
        self.errors.contains_key(key)
    }

    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
}

impl Default for ValidationErrors {
    fn default() -> Self {
        Self {
            default_value: "valeur incorrecte".to_owned(),
            errors: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let mut errors = ValidationErrors::default();
        errors.set("un", "my_error");
        errors.set_default("deux");

        assert_eq!(errors.get("un"), Some("my_error".to_owned()));
        assert_eq!(errors.get("deux"), Some("valeur incorrecte".to_owned()));
    }

    #[test]
    fn test_new() {
        let mut errors = ValidationErrors::new("bad");
        errors.set("un", "my_error");
        errors.set_default("deux");

        assert_eq!(errors.get("un"), Some("my_error".to_owned()));
        assert_eq!(errors.get("deux"), Some("bad".to_owned()));
    }
}
