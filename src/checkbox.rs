use maud::{html, Markup, Render};

#[derive(Clone, Debug, Default)]
pub enum CheckboxType {
    #[default]
    Check,

    Radio,
}

///Pour gérer l'attribut checked des checkbox
#[derive(Clone, Debug, Default)]
pub struct Checkbox {
    name: String,
    label: String,
    class: Option<String>,
    my_type: CheckboxType,
    is_checked: bool,
}

impl Checkbox {
    pub fn new(name: &str, label: &str) -> Self {
        Self {
            name: name.to_owned(),
            label: label.to_owned(),
            ..Default::default()
        }
    }

    pub fn new_radio(name: &str, label: &str) -> Self {
        Self {
            name: name.to_owned(),
            label: label.to_owned(),
            my_type: CheckboxType::Radio,
            ..Default::default()
        }
    }

    pub fn class(self, class: &str) -> Self {
        Self {
            class: Some(class.to_owned()),
            ..self
        }
    }

    pub fn checked(self, is_checked: bool) -> Self {
        Self { is_checked, ..self }
    }
}

impl Render for Checkbox {
    fn render(&self) -> Markup {
        let css = format!(
            "form-check{}",
            self.class
                .as_ref()
                .map(|x| format!(" {x}"))
                .unwrap_or_default()
        );
        let type_str = match self.my_type {
            CheckboxType::Check => "checkbox",
            CheckboxType::Radio => "radio",
        };

        html!(
            div class=(css) {
                input
                    name=(self.name)
                    class="form-check-input"
                    type=(type_str)
                    checked[self.is_checked];
                label class="form-check-label" {(self.label)}
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use maud::Render;

    use crate::checkbox::Checkbox;

    #[test]
    fn checkbox_default() {
        let on = Checkbox::new("cbx", "Choisir");

        assert_eq!(
            on.render().into_string(),
            concat!(
                r#"<div class="form-check">"#,
                r#"<input name="cbx" class="form-check-input" type="checkbox">"#,
                r#"<label class="form-check-label">Choisir</label>"#,
                r#"</div>"#,
            )
        );
    }

    #[test]
    fn checkbox_checked() {
        let on = Checkbox::new("cbx", "Choisir").checked(true);

        assert_eq!(
            on.render().into_string(),
            concat!(
                r#"<div class="form-check">"#,
                r#"<input name="cbx" class="form-check-input" type="checkbox" checked>"#,
                r#"<label class="form-check-label">Choisir</label>"#,
                r#"</div>"#,
            )
        );
    }
}
