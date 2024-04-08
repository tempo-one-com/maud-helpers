#[derive(Clone, Debug, Default)]
pub struct Props {
    pub value: Option<String>,
    pub id: Option<String>,
    pub hint: Option<String>,
    pub required: bool,
    pub placeholder: Option<String>,
    pub error: Option<String>,
}

impl Props {
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
