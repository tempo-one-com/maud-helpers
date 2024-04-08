use maud::{html, Markup, Render};

use crate::{
    error::ValidationErrors,
    field_props::Props,
    key_value::{KeyValue, KeyValueInterface},
};

#[derive(Clone, Debug, Default)]
pub struct Select {
    name: String,
    label: String,
    class: String,
    items: Vec<KeyValue>,
    props: Props,
    errors: ValidationErrors,
}

impl Select {
    ///Gestion des select/option
    pub fn simple<A>(name: &str, label: &str, items: &[A]) -> Self
    where
        A: KeyValueInterface,
    {
        let kvs = items.iter().map(|x| x.to_kv()).collect::<Vec<_>>();

        Self {
            name: name.to_owned(),
            label: label.to_owned(),
            items: kvs,
            class: "form-floating".to_owned(),
            ..Default::default()
        }
    }

    pub fn new_kv(name: &str, label: &str, items: &[KeyValue]) -> Self {
        Self {
            name: name.to_owned(),
            label: label.to_owned(),
            items: items.to_owned(),
            class: "form-floating".to_owned(),
            ..Default::default()
        }
    }

    pub fn props(self, props: Props) -> Self {
        Self { props, ..self }
    }

    ///Pour gérer l'attribut l'option a sélectionner dans un select
    pub fn checked_option<S: Into<String>>(&self, reference: Option<S>, value: &str) -> bool {
        reference
            .map(|x| value.to_string() == x.into())
            .unwrap_or_default()
    }

    pub fn class(self, class: &str) -> Self {
        Self {
            class: format!("{} {class}", self.class),
            ..self
        }
    }

    pub fn errors(self, errors: &ValidationErrors) -> Self {
        Self {
            errors: errors.to_owned(),
            ..self
        }
    }
}

impl Render for Select {
    fn render(&self) -> Markup {
        html!(
            div class=(self.class) {
            select
                name=(self.name)
                class="form-select"
                id=[self.props.clone().id]
               {
            @for item in &self.items {
                option
                    value=(item.key)
                    selected[(self.checked_option(self.props.value.clone(), &item.key))]  {(item.value)};
            }}
            label {(self.label)}
            @if let Some(error) = self.errors.get(&self.name) {
                div class="invalid-feedback" {(error)}
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use maud::{html, Render};

    use crate::{
        error::ValidationErrors,
        field_props::Props,
        key_value::{KeyValue, KeyValueInterface},
        select::Select,
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
        let select = Select::simple("", "", &vec![Toto::new(1, "")]);
        let matching_selection = select.checked_option(Some("toto"), "toto");
        let none_matching_selection = select.checked_option(Some("toto".to_string()), "titi");

        assert!(matching_selection);
        assert!(!none_matching_selection);
    }

    #[test]
    fn select_tag_with_no_selected() {
        let items = vec![Toto::new(1, "A"), Toto::new(2, "B")];

        let with_selected_option =
            html!((Select::simple("mon_select", "choisir", &items).class("mb-4")));

        assert_eq!(
            with_selected_option.into_string(),
            concat!(
                r#"<div class="form-floating mb-4">"#,
                r#"<select name="mon_select" class="form-select">"#,
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

        let with_selected_option = html!(
            (Select::simple("mon_select", "choisir", &items).props(Props::default().value("2")))
        );

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
            (Select::new_kv("mon_select", "choisir", &items)
                .props(Props::default().value("2").id("my_id"))
                .class("c"))
        );

        assert_eq!(
            with_selected_option.into_string(),
            concat!(
                r#"<div class="form-floating c">"#,
                r#"<select name="mon_select" class="form-select" id="my_id">"#,
                r#"<option value="1">A</option>"#,
                r#"<option value="2" selected>B</option>"#,
                r#"</select>"#,
                r#"<label>choisir</label>"#,
                r#"</div>"#,
            )
        );
    }

    #[test]
    fn select_error() {
        let mut validation = ValidationErrors::default();
        validation.set_default("name");
        let select = Select::simple("name", "", &vec![Toto::new(1, "")]).errors(&validation);

        assert_eq!(
            select.render().into_string(),
            concat!(
                r#"<div class="form-floating">"#,
                r#"<select name="name" class="form-select">"#,
                r#"<option value="1"></option>"#,
                r#"</select>"#,
                r#"<label></label>"#,
                r#"<div class="invalid-feedback">valeur incorrecte</div>"#,
                r#"</div>"#,
            )
        )
    }
}
