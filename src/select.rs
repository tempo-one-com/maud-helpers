use maud::{html, Markup};

use crate::key_value::{KeyValue, KeyValueInterface};

///Pour gérer l'attribut l'option a sélectionner dans un select
pub fn checked_option(reference: Option<KeyValue>, value: &str) -> bool {
    reference.map(|x| x.key == value).unwrap_or_default()
}

///Gestion des select/option
pub fn select<A>(name: &str, class: Option<&str>, items: &[A], selected: Option<KeyValue>) -> Markup
where
    A: KeyValueInterface,
{
    let tuples = items.iter().map(|x| x.to_kv()).collect::<Vec<_>>();

    build_select(name, class, &tuples, selected)
}

fn build_select(
    name: &str,
    class: Option<&str>,
    items: &[KeyValue],
    selected: Option<KeyValue>,
) -> Markup {
    let css = class.map(|x| format!(" {x}")).unwrap_or_default();

    html!(
        select name=(name) class={"form-select" (css)} {
        @for item in items {
            option value=(item.key) selected[(checked_option(selected.clone(), &item.key))]  {(item.value)};
        }
    })
}

#[cfg(test)]
mod tests {
    use maud::html;

    use crate::{
        key_value::{KeyValue, KeyValueInterface},
        select::{checked_option, select},
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
        let no_selection = checked_option(None, "toto");
        let matching_selection = checked_option(Some(KeyValue::new_key("toto")), "toto");
        let none_matching_selection = checked_option(Some(KeyValue::new_key("toto")), "titi");

        assert!(!no_selection);
        assert!(matching_selection);
        assert!(!none_matching_selection);
    }

    #[test]
    fn select_tag_with_no_selected() {
        let items = vec![Toto::new(1, "A"), Toto::new(2, "B")];

        let with_selected_option =
            html!((select("mon_select", Some("selected bordered"), &items, None)));

        assert_eq!(
            with_selected_option.into_string(),
            concat!(
                r#"<select name="mon_select" class="form-select selected bordered">"#,
                r#"<option value="1">A</option>"#,
                r#"<option value="2">B</option>"#,
                r#"</select>"#,
            )
        );
    }

    #[test]
    fn select_tag_with_selected() {
        let items = vec![Toto::new(1, "A"), Toto::new(2, "B")];

        let with_selected_option =
            html!((select("mon_select", None, &items, Some(KeyValue::new_key("2")))));

        assert_eq!(
            with_selected_option.into_string(),
            concat!(
                r#"<select name="mon_select" class="form-select">"#,
                r#"<option value="1">A</option>"#,
                r#"<option value="2" selected>B</option>"#,
                r#"</select>"#,
            )
        );
    }

    // #[test]
    // fn select_tag_with_bad_selected() {
    //     let items: Vec<dyn KeyValueInterface> = vec![
    //         Toto {
    //             id: 1,
    //             code: "A".to_owned(),
    //         },
    //         Toto {
    //             id: 2,
    //             code: "B".to_owned(),
    //         },
    //     ];

    //     let with_selected_option = html!((select("mon_select", Some("c"), &items, Some("3"))));

    //     assert_eq!(
    //         with_selected_option.into_string(),
    //         concat!(
    //             r#"<select name="mon_select" class="c">"#,
    //             r#"<option value="1">A</option>"#,
    //             r#"<option value="2">B</option>"#,
    //             r#"</select>"#,
    //         )
    //     );
    // }

    // #[test]
    // fn select_tag_with_string() {
    //     let items = vec![KeyValue::new("1", "A"), KeyValue::new("2", "B")];

    //     let with_selected_option = html!((select("mon_select", Some("c"), &items, Some("3"))));

    //     assert_eq!(
    //         with_selected_option.into_string(),
    //         concat!(
    //             r#"<select name="mon_select" class="c">"#,
    //             r#"<option value="1">A</option>"#,
    //             r#"<option value="2">B</option>"#,
    //             r#"</select>"#,
    //         )
    //     );
    // }
}
