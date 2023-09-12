use maud::{html, Markup};

///Pour gérer l'attribut l'option a sélectionner dans un select
pub fn checked_option(value_to_match: Option<&str>, value: &str) -> Option<bool> {
    match value_to_match {
        Some(v) if v == value => Some(true),
        _ => None,
    }
}

// pub fn select(name: &str, class: &str, items: &[&str], selected: Option<&str>) -> Markup {
//     html!(
//         select name=(name) class=(class) {
//         @for item in items {
//             option value=(item) selected=[checked_option(selected, item)] {(item)};
//         }
//     })
// }

pub fn select(name: &str, class: &str, items: &[String], selected: Option<&str>) -> Markup {
    html!(
       select name=(name) class=(class) {
       @for item in items {
          option value=(item) selected=[checked_option(selected, item)] {(item)};
       }
    })
}

// pub fn select<T>(name: &str, class: &str, items: &Vec<T>, selected: Option<&str>) -> Markup
// where
//     T: Into<String>,
//  {
//     html!(
//         select name=(name) class=(class) {
//         @for item in items {
//             @let s: String = item.into();
//             option value=(s) selected=[checked_option(selected, &s)] {(s)};
//         }
//     })
// }

#[cfg(test)]
mod tests {
    use maud::html;

    use crate::select::{select, checked_option};

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
        let items = vec!["1".to_owned(), "2".to_owned()];

        let with_selected_option = html!((select("mon_select", "selected bordered", &items, None)));

        assert_eq!(
            with_selected_option.into_string(),
            concat!(
                r#"<select name="mon_select" class="selected bordered">"#,
                r#"<option value="1">1</option>"#,
                r#"<option value="2">2</option>"#,
                r#"</select>"#,
            )
        );
    }

    #[test]
    fn select_tag_with_selected() {
        let items = vec!["1".to_owned(), "2".to_owned()];

        let with_selected_option =
            html!((select("mon_select", "selected bordered", &items, Some("2"))));

        assert_eq!(
            with_selected_option.into_string(),
            concat!(
                r#"<select name="mon_select" class="selected bordered">"#,
                r#"<option value="1">1</option>"#,
                r#"<option value="2" selected="true">2</option>"#,
                r#"</select>"#,
            )
        );
    }

    #[test]
    fn select_tag_with_bad_selected() {
        let items = vec!["1".to_owned(), "2".to_owned()];

        let with_selected_option =
            html!((select("mon_select", "selected bordered", &items, Some("3"))));

        assert_eq!(
            with_selected_option.into_string(),
            concat!(
                r#"<select name="mon_select" class="selected bordered">"#,
                r#"<option value="1">1</option>"#,
                r#"<option value="2">2</option>"#,
                r#"</select>"#,
            )
        );
    }
}
