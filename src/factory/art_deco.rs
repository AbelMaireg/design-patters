use super::{ChairFactory, FurnitureFactory, TableFactory};

pub struct ArtDecoChair;
pub struct ArtDecoTable;
pub struct ArtDecoFurniture;

impl ChairFactory for ArtDecoChair {
    fn operation(&self) -> String {
        "art deco chair".to_string()
    }
}

impl TableFactory for ArtDecoTable {
    fn operation(&self) -> String {
        "art deco table".to_string()
    }
}

impl FurnitureFactory for ArtDecoFurniture {
    fn create_chair(&self) -> Box<dyn ChairFactory> {
        Box::new(ArtDecoChair)
    }

    fn create_table(&self) -> Box<dyn TableFactory> {
        Box::new(ArtDecoTable)
    }
}
