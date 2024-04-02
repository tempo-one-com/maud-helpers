use maud::{html, Markup, Render};

use crate::tag_options::TagOptions;

#[derive(Clone, Debug, Default)]
pub enum TextType {
    #[default]
    Text,

    Email,
    Number,
}

#[derive(Clone, Debug, Default)]
pub struct Text {
    name: String,
    label: String,
    my_type: TextType,
    value: Option<String>,
    class: Option<String>,
    options: TagOptions,
}

impl Text {
    pub fn new(name: &str, label: &str, value: Option<&str>) -> Self {
        Self {
            name: name.to_owned(),
            label: label.to_owned(),
            value: value.map(ToOwned::to_owned),
            ..Default::default()
        }
    }

    pub fn text_type(self, new_type: TextType) -> Self {
        Self {
            my_type: new_type,
            ..self
        }
    }

    pub fn class(self, class: &str) -> Self {
        Self {
            class: Some(class.to_owned()),
            ..self
        }
    }

    pub fn options(self, options: TagOptions) -> Self {
        Self { options, ..self }
    }
}

impl Render for Text {
    fn render(&self) -> Markup {
        let css = format!(
            "form-floating{}",
            self.class
                .as_ref()
                .map(|x| format!(" {x}"))
                .unwrap_or_default()
        );

        let type_str = match self.my_type {
            TextType::Text => "text",
            TextType::Email => "email",
            TextType::Number => "number",
        };

        html!(
            div class=(css) {
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
        let text = Text::new("name", "Name", None);

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
        let text = Text::new("name", "Name", None).class("mb-3");

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
        let text = Text::new("name", "Name", None).class("mb-3").options(opt);

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
        let text = Text::new("name", "Name", None).class("mb-3").options(opt);

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
}
