use maud::{html, Markup, Render};

use crate::tag_options::TagOptions;

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
    value: Option<String>,
    class: String,
    options: TagOptions,
}

impl TextField {
    fn new(my_type: TextFieldType, name: &str, label: &str, value: Option<&str>) -> Self {
        Self {
            name: name.to_owned(),
            label: label.to_owned(),
            value: value.map(ToOwned::to_owned),
            class: "form-floating".to_owned(),
            my_type,
            ..Default::default()
        }
    }

    pub fn text(name: &str, label: &str, value: Option<&str>) -> Self {
        Self::new(TextFieldType::Text, name, label, value)
    }

    pub fn email(name: &str, label: &str, value: Option<&str>) -> Self {
        Self::new(TextFieldType::Email, name, label, value)
    }

    pub fn number(name: &str, label: &str, value: Option<&str>) -> Self {
        Self::new(TextFieldType::Number, name, label, value)
    }

    pub fn class(self, class: &str) -> Self {
        Self {
            class: format!("{} {class}", self.class),
            ..self
        }
    }

    pub fn options(self, options: TagOptions) -> Self {
        Self { options, ..self }
    }
}

impl Render for TextField {
    fn render(&self) -> Markup {
        let type_str = match self.my_type {
            TextFieldType::Text => "text",
            TextFieldType::Email => "email",
            TextFieldType::Number => "number",
        };

        html!(
            div class=(self.class) {
                input
                    type=(type_str)
                    class="form-control"
                    name=(self.name)
                    id=[self.options.clone().id]
                    value=[self.value.clone()]
                    placeholder=[self.options.clone().placeholder];
                label {(self.label)}
                @if let Some(hint) = self.options.clone().hint {
                    div class="form-text" {(hint)}
                }
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let text = TextField::text("name", "Name", None);

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
        let text = TextField::text("name", "Name", None).class("mb-3");

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
        let opt = TagOptions::new().id("my_id").hint("indice");
        let text = TextField::text("name", "Name", None)
            .class("mb-3")
            .options(opt);

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
    fn test_placeholder() {
        let opt = TagOptions::new().hint("indice");
        let text = TextField::text("name", "Name", None)
            .class("mb-3")
            .options(opt);

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
        let text = TextField::email("name", "Name", None);

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
}
