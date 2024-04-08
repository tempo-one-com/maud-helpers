use crate::field_props::Props;

pub trait FieldInterface {
    fn props(self, value: Props) -> Self;
}
