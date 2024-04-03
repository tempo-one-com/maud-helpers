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
    class: String,
    my_type: CheckboxType,
    is_checked: bool,
}

impl Checkbox {
    fn new(my_type: CheckboxType, name: &str, label: &str) -> Self {
        Self {
            name: name.to_owned(),
            label: label.to_owned(),
            class: "form-check".to_owned(),
            my_type,
            ..Default::default()
        }
    }

    pub fn check(name: &str, label: &str) -> Self {
        Self::new(CheckboxType::Check, name, label)
    }

    pub fn radio(name: &str, label: &str) -> Self {
        Self::new(CheckboxType::Radio, name, label)
    }

    pub fn class(self, classes: &str) -> Self {
        let class = format!("{} {}", self.class, classes);

        Self { class, ..self }
    }

    pub fn checked(self, is_checked: bool) -> Self {
        Self { is_checked, ..self }
    }
}

impl Render for Checkbox {
    fn render(&self) -> Markup {
        let type_str = match self.my_type {
            CheckboxType::Check => "checkbox",
            CheckboxType::Radio => "radio",
        };

        html!(
            div class=(self.class) {
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
        let on = Checkbox::check("cbx", "Choisir");

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
        let on = Checkbox::check("cbx", "Choisir")
            .class("my-class")
            .checked(true);

        assert_eq!(
            on.render().into_string(),
            concat!(
                r#"<div class="form-check my-class">"#,
                r#"<input name="cbx" class="form-check-input" type="checkbox" checked>"#,
                r#"<label class="form-check-label">Choisir</label>"#,
                r#"</div>"#,
            )
        );
    }
}
