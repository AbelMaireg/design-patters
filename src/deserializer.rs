use crate::visitor::Visitor;

pub trait Deserializer<V: Visitor> {
    fn create(visitor: V) -> Self;

    fn parse_str(&self, _input: &str) -> Result<V::Value, &'static str> {
        Err("parse_str not implemented")
    }

    fn parse_vec(&self, _input: Vec<i32>) -> Result<V::Value, &'static str> {
        Err("parse_vec not implemented")
    }

    fn parse_rectangle(&self, _input: Rectangle) -> Result<V::Value, &'static str> {
        Err("parse_rectangle not implemented")
    }
}

pub struct StringDeserializer<V: Visitor> {
    pub visitor: V,
}

impl<V: Visitor> Deserializer<V> for StringDeserializer<V> {
    fn create(visitor: V) -> Self {
        Self { visitor }
    }

    fn parse_str(&self, input: &str) -> Result<V::Value, &'static str> {
        let input_vec = input
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        Ok(self.visitor.visit_vec(input_vec))
    }
}

pub struct VecDeserializer<V: Visitor> {
    pub visitor: V,
}

impl<V: Visitor> Deserializer<V> for VecDeserializer<V> {
    fn create(visitor: V) -> Self {
        Self { visitor }
    }

    fn parse_vec(&self, input: Vec<i32>) -> Result<V::Value, &'static str> {
        Ok(self.visitor.visit_vec(input))
    }
}

pub struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

pub struct RectangleDeserializer<V: Visitor> {
    visitor: V,
}

impl<V: Visitor> Deserializer<V> for RectangleDeserializer<V> {
    fn create(visitor: V) -> Self {
        Self { visitor }
    }

    fn parse_rectangle(&self, _input: Rectangle) -> Result<<V as Visitor>::Value, &'static str> {
        Ok(self.visitor.visit_vec(vec![_input.width, _input.height]))
    }
}
