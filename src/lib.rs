pub mod checkbox;
pub mod select;

pub trait IdValue {
    fn id(&self) -> String;
    fn value(&self) -> String;
}