use deserializer::{
    Deserializer, Rectangle, RectangleDeserializer, StringDeserializer, VecDeserializer,
};
use visitor::{TwoValueArray, TwoValueStruct};

pub mod deserializer;
pub mod visitor;

fn main() {
    let deserializer = StringDeserializer::create(TwoValueStruct::default());
    let result = deserializer.parse_str("123 456");
    println!("{:?}", result);

    let deserializer = VecDeserializer::create(TwoValueStruct::default());
    let result = deserializer.parse_vec(vec![123, 456]);
    println!("{:?}", result);

    let deserializer = VecDeserializer::create(TwoValueArray::default());
    let result = deserializer.parse_vec(vec![123, 456]);
    println!("{:?}", result);

    let deserializer = RectangleDeserializer::create(TwoValueStruct::default());
    let result = deserializer.parse_rectangle(Rectangle::new(3, 4));
    println!("{:?}", result);

    eprintln!("{}", deserializer.parse_str("123 456").err().unwrap())
}
