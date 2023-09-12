///Pour gérer l'attribut checked des checkbox
pub fn need_checked(value: Option<&str>) -> Option<bool> {
   match value {
       Some(x) if x == "on" => Some(true),
       _ => None
   }
}

#[cfg(test)]
mod tests {
    use crate::checkbox::need_checked;


    #[test]
    fn checkbox_works() {
        let on = need_checked(Some("on"));
        let off = need_checked(None);
        let off2 = need_checked(Some("any"));

        assert_eq!(on, Some(true));
        assert_eq!(off, None);
        assert_eq!(off2, None);        
    }
}