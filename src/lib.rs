pub mod checkbox;
pub mod select;

pub trait IdValue: Sized {
    fn id(&self) -> String;
    fn value(&self) -> String;
}