use maud::{html, Markup};

use crate::IdValue;

///Pour gérer l'attribut l'option a sélectionner dans un select
pub fn checked_option(value_to_match: Option<&str>, value: &str) -> Option<bool> {
    match value_to_match {
        Some(v) if v == value => Some(true),
        _ => None,
    }
}
///Gestion des select/option
pub fn select<A>(name: &str, class: &str, items: &[A], selected: Option<&str>) -> Markup 
where A: IdValue
{
    html!(
       select name=(name) class=(class) {
       @for item in items {
          option value=(&item.id()) selected=[checked_option(selected, &item.id())] {(item.value())};
       }
    })
}

#[cfg(test)]
mod tests {
    use maud::html;

    use crate::{select::{select, checked_option}, IdValue};

    struct Toto {
        id: i32,
        code: String
    }

    impl IdValue for Toto {
        fn id(&self) -> String {
            self.id.to_string()
        }

        fn value(&self) -> String {
            self.code.clone()
        }
    }    

    #[test]
    fn select_option() {
        let no_selection = checked_option(None, "toto");
        let matching_selection = checked_option(Some("toto"), "toto");
        let none_matching_selection = checked_option(Some("toto"), "titi");

        assert_eq!(no_selection, None);
        assert_eq!(matching_selection, Some(true));
        assert_eq!(none_matching_selection, None);
    }

    #[test]
    fn select_tag_with_no_selected() {
        let items = vec![
            Toto { id: 1, code: "A".to_owned()},
            Toto { id: 2, code: "B".to_owned()},
        ];

        let with_selected_option = html!((select("mon_select", "selected bordered", &items, None)));

        assert_eq!(
            with_selected_option.into_string(),
            concat!(
                r#"<select name="mon_select" class="selected bordered">"#,
                r#"<option value="1">A</option>"#,
                r#"<option value="2">B</option>"#,
                r#"</select>"#,
            )
        );
    }

    #[test]
    fn select_tag_with_selected() {
        let items = vec![
            Toto { id: 1, code: "A".to_owned()},
            Toto { id: 2, code: "B".to_owned()},
        ];

        let with_selected_option =
            html!((select("mon_select", "c", &items, Some("2"))));

        assert_eq!(
            with_selected_option.into_string(),
            concat!(
                r#"<select name="mon_select" class="c">"#,
                r#"<option value="1">A</option>"#,
                r#"<option value="2" selected="true">B</option>"#,
                r#"</select>"#,
            )
        );
    }

    #[test]
    fn select_tag_with_bad_selected() {
        let items = vec![
            Toto { id: 1, code: "A".to_owned()},
            Toto { id: 2, code: "B".to_owned()},
        ];

        let with_selected_option =
            html!((select("mon_select", "c", &items, Some("3"))));

        assert_eq!(
            with_selected_option.into_string(),
            concat!(
                r#"<select name="mon_select" class="c">"#,
                r#"<option value="1">A</option>"#,
                r#"<option value="2">B</option>"#,
                r#"</select>"#,
            )
        );
    }
}
