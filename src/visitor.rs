pub trait Visitor {
    type Value;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value;
}

#[derive(Debug, Default)]
pub struct TwoValueStruct {
    a: i32,
    b: i32,
}

impl Visitor for TwoValueStruct {
    type Value = TwoValueStruct;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        TwoValueStruct { a: v[0], b: v[1] }
    }
}

#[derive(Debug, Default)]
pub struct TwoValueArray([i32; 2]);

impl Visitor for TwoValueArray {
    type Value = TwoValueArray;

    fn visit_vec(&self, v: Vec<i32>) -> Self::Value {
        TwoValueArray([v[0], v[1]])
    }
}
