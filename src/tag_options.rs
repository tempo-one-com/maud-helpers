#[derive(Clone, Debug, Default)]
pub struct TagOptions {
    pub id: Option<String>,
    pub placeholder: Option<String>,
    pub hint: Option<String>,
    pub required: bool,
}

impl TagOptions {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn id(self, value: &str) -> Self {
        Self {
            id: Some(value.to_owned()),
            ..self
        }
    }

    pub fn placeholder(self, value: &str) -> Self {
        Self {
            placeholder: Some(value.to_owned()),
            ..self
        }
    }

    pub fn hint(self, value: &str) -> Self {
        Self {
            hint: Some(value.to_owned()),
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
