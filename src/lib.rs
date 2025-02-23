pub mod factory;

#[cfg(test)]
mod test {
    use crate::factory::ArtDecoFurniture;
    use crate::factory::FurnitureFactory;
    use crate::factory::ModernFurniture;

    #[test]
    fn test_modern_factory() {
        let factory = ModernFurniture;
        let chair = factory.create_chair();
        let table = factory.create_table();
        assert_eq!(chair.operation(), "modern chair");
        assert_eq!(table.operation(), "modern table");
    }

    #[test]
    fn test_art_deco_factory() {
        let factory = ArtDecoFurniture;
        let chair = factory.create_chair();
        let table = factory.create_table();
        assert_eq!(chair.operation(), "art deco chair");
        assert_eq!(table.operation(), "art deco table");
    }
}
