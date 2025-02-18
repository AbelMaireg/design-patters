pub trait Command {
    type Type;
    fn execute(&self) -> Self::Type;
}
