#[allow(dead_code)]
pub trait Iterator<T> {
    type Item;
    fn next(&mut self) -> Option<T>;
    fn size_hint(&self) -> (usize, Option<usize>);
}

pub trait IntoIteratorII<T> {
    type Item;
    type IntoIter;
    fn into_iterII(self) -> Self::IntoIter;
}
