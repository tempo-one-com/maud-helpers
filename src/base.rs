use maud::{html, Markup, Render};

#[derive(Clone, Debug, Default)]
pub struct Row<R> {
    class: String,
    items: Vec<R>,
}

impl<R: Default + Clone> Row<R> {
    pub fn new() -> Self {
        Self {
            class: "row".to_owned(),
            ..Default::default()
        }
    }

    pub fn class(self, class: &str) -> Self {
        Self {
            class: format!("{} {}", self.class, class),
            ..self
        }
    }

    pub fn add(self, item: R) -> Self
    where
        R: Clone + Default,
    {
        let mut items = self.items;
        items.push(item);

        Self {
            items: items.clone(),
            ..self
        }
    }
}

impl<R: Clone + Render> Render for Row<R> {
    fn render(&self) -> Markup {
        html!(
            div class=(self.class) {
                @for item in self.items.clone() {
                    (item.render())
                }
            }
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct Col<R> {
    class: String,
    items: Vec<R>,
}

impl<R: Default> Col<R> {
    pub fn new() -> Self {
        Self {
            class: "col".to_owned(),
            ..Default::default()
        }
    }

    pub fn class(self, class: &str) -> Self {
        Self {
            class: format!("{} {}", self.class, class),
            ..self
        }
    }

    pub fn add(self, item: R) -> Self
    where
        R: Clone + Render,
    {
        let mut items = self.items;
        items.push(item);

        Self {
            items: items.clone(),
            ..self
        }
    }
}

impl<R: Clone + Render> Render for Col<R> {
    fn render(&self) -> Markup {
        html!(
            div class=(self.class) {
                @for item in self.items.clone() {
                    (item.render())
                }
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use maud::Render;

    use crate::text::TextField;

    use super::*;

    #[test]
    fn test() {
        let text = TextField::text("name", "label", Some("1"));
        let text2 = TextField::text("name", "label2", None);
        let div = Row::new().class("mb-4").add(
            Col::new()
                .class("col-md-2")
                .add(text.render())
                .add(text2.render()),
        );

        assert_eq!(
            div.render().into_string(),
            concat!(
                r#"<div class="row mb-4">"#,
                r#"<div class="col col-md-2">"#,
                r#"<div class="form-floating"><input type="text" class="form-control" name="name" value="1"><label>label</label></div>"#,
                r#"<div class="form-floating"><input type="text" class="form-control" name="name"><label>label2</label></div>"#,
                r#"</div>"#,
                r#"</div>"#,
            )
        );
    }
}
