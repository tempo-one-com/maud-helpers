use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ValidationErrors {
    errors: HashMap<String, Option<String>>,
}

impl ValidationErrors {
    pub fn mark(&mut self, key: &str) {
        self.errors.insert(key.into(), None);
    }

    pub fn mark_message<S: Into<String>>(&mut self, key: S, value: S) {
        self.errors.insert(key.into(), Some(value.into()));
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.errors.get(key).cloned().flatten()
    }

    pub fn has(&self, key: &str) -> bool {
        self.errors.contains_key(key)
    }

    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let mut errors = ValidationErrors::default();
        errors.mark_message("un", "my_error");
        errors.mark("deux");

        assert_eq!(errors.get("un"), Some("my_error".to_owned()));
        assert!(errors.get("deux").is_none());
    }

    #[test]
    fn test_new() {
        let mut errors = ValidationErrors::default();
        errors.mark("un");
        errors.mark_message("deux", "bad");

        assert!(errors.get("un").is_none());
        assert_eq!(errors.get("deux"), Some("bad".to_owned()));
    }
}
