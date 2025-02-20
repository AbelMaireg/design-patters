pub mod deserializer;
pub mod visitor;

#[cfg(test)]
mod test {
    use super::deserializer::{
        Deserializer, Rectangle, RectangleDeserializer, StringDeserializer, VecDeserializer,
    };
    use super::visitor::{TwoValueArray, TwoValueStruct};

    #[test]
    fn test_string_deserializer() {
        let deserializer = StringDeserializer::create(TwoValueStruct::default());

        assert_eq!(
            deserializer.parse_str("123 456"),
            Ok(TwoValueStruct::new(123, 456))
        );

        let deserializer = StringDeserializer::create(TwoValueArray::default());

        assert_eq!(
            deserializer.parse_str("123 456"),
            Ok(TwoValueArray::new(123, 456))
        );
    }

    #[test]
    fn test_vector_deserializer() {
        let deserializer = VecDeserializer::create(TwoValueStruct::default());

        assert_eq!(
            deserializer.parse_vec(vec![123, 456]),
            Ok(TwoValueStruct::new(123, 456))
        );

        let deserializer = VecDeserializer::create(TwoValueArray::default());

        assert_eq!(
            deserializer.parse_vec(vec![123, 456]),
            Ok(TwoValueArray::new(123, 456))
        );
    }

    #[test]
    fn test_rectangle_deserializer() {
        let deserializer = RectangleDeserializer::create(TwoValueStruct::default());

        assert_eq!(
            deserializer.parse_rectangle(Rectangle::new(3, 4)),
            Ok(TwoValueStruct::new(3, 4))
        );

        let deserializer = RectangleDeserializer::create(TwoValueArray::default());

        assert_eq!(
            deserializer.parse_rectangle(Rectangle::new(3, 4)),
            Ok(TwoValueArray::new(3, 4))
        );
    }

    #[test]
    fn returns_error() {
        let deserializer = RectangleDeserializer::create(TwoValueStruct::default());

        assert!(deserializer.parse_str("123 456").is_err());

        assert!(deserializer.parse_vec(vec![123, 456]).is_err());
    }
}
