use maud::{html, Markup};

#[derive(Clone, Debug, Default)]
pub struct TextOptions {
    id: Option<String>,
    placeholder: Option<String>,
    hint: Option<String>,
}

impl TextOptions {
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
}

pub fn text(name: &str, label: &str, value: Option<&str>, class: Option<&str>) -> Markup {
    text_opt(name, label, value, class, TextOptions::new())
}

pub fn text_opt(
    name: &str,
    label: &str,
    value: Option<&str>,
    class: Option<&str>,
    options: TextOptions,
) -> Markup {
    let class = format!(
        "form-floating{}",
        class.map(|x| format!(" {x}")).unwrap_or_default()
    );

    html!(
        div class=(class) {
            input type="text" class="form-control"
                name=(name)
                id=[options.id]
                value=[value]
                placeholder=[options.placeholder];
            label {(label)}
            @if let Some(hint) = options.hint {
                div class="form-text" {(hint)}
            }
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let text = text("name", "Name", None, None);

        assert_eq!(
            text.into_string(),
            concat!(
                r#"<div class="form-floating">"#,
                r#"<input type="text" class="form-control" name="name">"#,
                r#"<label>Name</label></div>"#
            )
        );
    }

    #[test]
    fn test_with_class() {
        let text = text("name", "Name", None, Some("mb-3"));

        assert_eq!(
            text.into_string(),
            concat!(
                r#"<div class="form-floating mb-3">"#,
                r#"<input type="text" class="form-control" name="name">"#,
                r#"<label>Name</label></div>"#
            )
        );
    }

    #[test]
    fn test_id_and_hint() {
        let opt = TextOptions::new().id("my_id").hint("indice");
        let text = text_opt("name", "Name", None, Some("mb-3"), opt);

        assert_eq!(
            text.into_string(),
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
        let opt = TextOptions::new().hint("indice");
        let text = text_opt("name", "Name", None, Some("mb-3"), opt);

        assert_eq!(
            text.into_string(),
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
