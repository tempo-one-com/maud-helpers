#[derive(Clone, Debug, Default)]
pub struct Props {
    pub value: Option<String>,
    pub id: Option<String>,
    pub hint: Option<String>,
    pub required: bool,
    pub placeholder: Option<String>,
    pub error: Option<String>,
}

pub type DynOptionalString = dyn Into<String>;

impl Props {
    pub fn new_value<S: Into<String>>(value: Option<S>) -> Self {
        Self {
            value: value.map(Into::into),
            ..Default::default()
        }
    }

    pub fn value(self, value: &str) -> Self {
        Self {
            value: Some(value.to_owned()),
            ..self
        }
    }

    pub fn id(self, value: &str) -> Self {
        Self {
            id: Some(value.to_owned()),
            ..self
        }
    }

    pub fn hint(self, value: &str) -> Self {
        Self {
            hint: Some(value.to_owned()),
            ..self
        }
    }

    pub fn placeholder(self, value: &str) -> Self {
        Self {
            placeholder: Some(value.to_owned()),
            ..self
        }
    }

    pub fn required(self) -> Self {
        Self {
            required: true,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_str() {
        let p = Props::new_value(Some("v"));
        let default = Props {
            value: Some("v".to_owned()),
            ..Default::default()
        };
        assert_eq!(default.value, p.value);
    }

    #[test]
    fn new_string() {
        let p = Props::new_value(Some("v".to_owned()));
        let default = Props {
            value: Some("v".to_owned()),
            ..Default::default()
        };
        assert_eq!(default.value, p.value);
    }
}
