use maud::{html, Markup, Render};

use crate::{
    key_value::{KeyValue, KeyValueInterface},
    tag_options::TagOptions,
};

#[derive(Clone, Debug, Default)]
pub struct Select {
    name: String,
    label: String,
    items: Vec<KeyValue>,
    value: Option<String>,
    class: Option<String>,
    options: TagOptions,
}

impl Select {
    ///Gestion des select/option
    pub fn new<A, S>(name: &str, label: &str, items: &[A], value: Option<S>) -> Self
    where
        A: KeyValueInterface,
        S: Into<String>,
    {
        let kvs = items.iter().map(|x| x.to_kv()).collect::<Vec<_>>();

        Self {
            name: name.to_owned(),
            label: label.to_owned(),
            items: kvs,
            value: value.map(Into::into),
            ..Default::default()
        }
    }

    pub fn new_kv<S: Into<String>>(
        name: &str,
        label: &str,
        items: &[KeyValue],
        value: Option<S>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            label: label.to_owned(),
            items: items.to_owned(),
            value: value.map(Into::into),
            ..Default::default()
        }
    }

    pub fn options(self, options: TagOptions) -> Self {
        Self { options, ..self }
    }

    ///Pour gérer l'attribut l'option a sélectionner dans un select
    pub fn checked_option<S: Into<String>>(&self, reference: Option<S>, value: &str) -> bool {
        reference
            .map(|x| value.to_string() == x.into())
            .unwrap_or_default()
    }

    pub fn class(self, class: &str) -> Self {
        Self {
            class: Some(class.to_owned()),
            ..self
        }
    }
}

impl Render for Select {
    fn render(&self) -> Markup {
        let css = self
            .class
            .as_ref()
            .map(|x| format!(" {x}"))
            .unwrap_or_default();

        html!(
            div class="form-floating" {
            select
                name=(self.name)
                class={"form-select" (css)}
                id=[self.options.clone().id]
               {
            @for item in &self.items {
                option
                    value=(item.key)
                    selected[(self.checked_option(self.value.clone(), &item.key))]  {(item.value)};
            }}
            label {(self.label)}
        })
    }
}

#[cfg(test)]
mod tests {
    use maud::html;

    use crate::{
        key_value::{KeyValue, KeyValueInterface},
        select::Select,
        tag_options::TagOptions,
    };

    struct Toto {
        id: i32,
        code: String,
    }

    impl Toto {
        fn new(id: i32, code: &str) -> Self {
            Self {
                id,
                code: code.to_owned(),
            }
        }
    }

    impl KeyValueInterface for Toto {
        fn to_kv(&self) -> KeyValue {
            KeyValue {
                key: self.id.to_string(),
                value: self.code.clone(),
            }
        }
    }

    #[test]
    fn select_option() {
        let select = Select::new("", "", &vec![Toto::new(1, "")], None::<String>);
        let matching_selection = select.checked_option(Some("toto"), "toto");
        let none_matching_selection = select.checked_option(Some("toto".to_string()), "titi");

        assert!(matching_selection);
        assert!(!none_matching_selection);
    }

    #[test]
    fn select_tag_with_no_selected() {
        let items = vec![Toto::new(1, "A"), Toto::new(2, "B")];

        let with_selected_option = html!(
            (Select::new("mon_select", "choisir", &items, None::<String>)
                .class("selected bordered"))
        );

        assert_eq!(
            with_selected_option.into_string(),
            concat!(
                r#"<div class="form-floating">"#,
                r#"<select name="mon_select" class="form-select selected bordered">"#,
                r#"<option value="1">A</option>"#,
                r#"<option value="2">B</option>"#,
                r#"</select>"#,
                r#"<label>choisir</label>"#,
                r#"</div>"#,
            )
        );
    }

    #[test]
    fn select_tag_with_selected() {
        let items = vec![Toto::new(1, "A"), Toto::new(2, "B")];

        let with_selected_option = html!((Select::new("mon_select", "choisir", &items, Some("2"))));

        assert_eq!(
            with_selected_option.into_string(),
            concat!(
                r#"<div class="form-floating">"#,
                r#"<select name="mon_select" class="form-select">"#,
                r#"<option value="1">A</option>"#,
                r#"<option value="2" selected>B</option>"#,
                r#"</select>"#,
                r#"<label>choisir</label>"#,
                r#"</div>"#,
            )
        );
    }

    #[test]
    fn select_tag_with_string() {
        let items = vec![KeyValue::new("1", "A"), KeyValue::new("2", "B")];

        let with_selected_option = html!(
            (Select::new_kv("mon_select", "choisir", &items, Some("2".to_string()))
                .class("c")
                .options(TagOptions::new().id("my_id")))
        );

        assert_eq!(
            with_selected_option.into_string(),
            concat!(
                r#"<div class="form-floating">"#,
                r#"<select name="mon_select" class="form-select c" id="my_id">"#,
                r#"<option value="1">A</option>"#,
                r#"<option value="2" selected>B</option>"#,
                r#"</select>"#,
                r#"<label>choisir</label>"#,
                r#"</div>"#,
            )
        );
    }
}
