use maud::{html, Markup, Render};
use validator::ValidationErrors;

use crate::field_props::Props;

#[derive(Clone, Debug, Default)]
pub enum TextFieldType {
    #[default]
    Text,

    Email,
    Number,
}

#[derive(Clone, Debug, Default)]
pub struct TextField {
    name: String,
    label: String,
    my_type: TextFieldType,
    class: String,
    props: Props,
    error: Option<String>,
}

impl TextField {
    pub fn text(name: &str, label: &str) -> Self {
        Self::new(TextFieldType::Text, name, label)
    }

    pub fn email(name: &str, label: &str) -> Self {
        Self::new(TextFieldType::Email, name, label)
    }

    pub fn number(name: &str, label: &str) -> Self {
        Self::new(TextFieldType::Number, name, label)
    }

    fn new(my_type: TextFieldType, name: &str, label: &str) -> Self {
        Self {
            name: name.to_owned(),
            label: label.to_owned(),
            class: "form-floating".to_owned(),
            my_type,
            ..Default::default()
        }
    }

    pub fn props(self, props: Props) -> Self {
        Self { props, ..self }
    }

    pub fn class(self, class: &str) -> Self {
        Self {
            class: format!("{} {class}", self.class),
            ..self
        }
    }

    pub fn errors(self, validation: &ValidationErrors) -> Self {
        Self {
            error: validation
                .errors()
                .get(self.name.as_str())
                .map(|_| "".to_string()),
            ..self
        }
    }
}

impl Render for TextField {
    fn render(&self) -> Markup {
        let type_str = match self.my_type {
            TextFieldType::Text => "text",
            TextFieldType::Email => "email",
            TextFieldType::Number => "number",
        };

        let class = self
            .error
            .clone()
            .map(|_| " is-invalid".to_string())
            .unwrap_or_default();

        html!(
            div class=(self.class) {
                input
                    type=(type_str)
                    class={"form-control"(class)}
                    name=(self.name)
                    id=[self.props.clone().id]
                    value=[self.props.value.clone()]
                    placeholder=[self.props.clone().placeholder];
                label {(self.label)}
                @if let Some(hint) = self.props.clone().hint {
                    div class="form-text" {(hint)}
                }
            }
        )
    }
}

#[cfg(test)]
mod tests {

    use validator::Validate;

    use super::*;

    #[derive(Validate)]
    struct Toto {
        #[validate(range(min = 1))]
        id: i32,
        code: String,
    }

    #[test]
    fn test_empty() {
        let text = TextField::text("name", "Name");

        assert_eq!(
            text.render().into_string(),
            concat!(
                r#"<div class="form-floating">"#,
                r#"<input type="text" class="form-control" name="name">"#,
                r#"<label>Name</label></div>"#
            )
        );
    }

    #[test]
    fn test_with_class() {
        let text = TextField::text("name", "Name").class("mb-3");

        assert_eq!(
            text.render().into_string(),
            concat!(
                r#"<div class="form-floating mb-3">"#,
                r#"<input type="text" class="form-control" name="name">"#,
                r#"<label>Name</label></div>"#
            )
        );
    }

    #[test]
    fn test_id_and_hint() {
        let text = TextField::text("name", "Name")
            .props(Props::default().id("my_id").hint("indice"))
            .class("mb-3");

        assert_eq!(
            text.render().into_string(),
            concat!(
                r#"<div class="form-floating mb-3">"#,
                r#"<input type="text" class="form-control" name="name" id="my_id">"#,
                r#"<label>Name</label>"#,
                r#"<div class="form-text">indice</div>"#,
                r#"</div>"#
            )
        );
    }

    #[test]
    fn test_hint() {
        let text = TextField::text("name", "Name")
            .class("mb-3")
            .props(Props::default().hint("indice"));

        assert_eq!(
            text.render().into_string(),
            concat!(
                r#"<div class="form-floating mb-3">"#,
                r#"<input type="text" class="form-control" name="name">"#,
                r#"<label>Name</label>"#,
                r#"<div class="form-text">indice</div>"#,
                r#"</div>"#
            )
        );
    }

    #[test]
    fn test_email() {
        let text = TextField::email("name", "Name");

        assert_eq!(
            text.render().into_string(),
            concat!(
                r#"<div class="form-floating">"#,
                r#"<input type="email" class="form-control" name="name">"#,
                r#"<label>Name</label>"#,
                r#"</div>"#
            )
        );
    }

    #[test]
    fn test_error() {
        let toto = Toto {
            id: 0,
            code: "".to_owned(),
        };

        let validation = toto.validate().err();
        let text = TextField::text("id", "Name")
            .props(Props::default())
            .errors(&validation.unwrap());

        assert_eq!(
            text.render().into_string(),
            concat!(
                r#"<div class="form-floating">"#,
                r#"<input type="text" class="form-control is-invalid" name="id">"#,
                r#"<label>Name</label>"#,
                r#"</div>"#
            )
        );
    }
}
