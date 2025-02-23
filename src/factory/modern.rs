use super::{ChairFactory, FurnitureFactory, TableFactory};

pub struct ModernChair;
pub struct ModernTable;
pub struct ModernFurniture;

impl ChairFactory for ModernChair {
    fn operation(&self) -> String {
        "modern chair".to_string()
    }
}

impl TableFactory for ModernTable {
    fn operation(&self) -> String {
        "modern table".to_string()
    }
}

impl FurnitureFactory for ModernFurniture {
    fn create_chair(&self) -> Box<dyn ChairFactory> {
        Box::new(ModernChair)
    }

    fn create_table(&self) -> Box<dyn TableFactory> {
        Box::new(ModernTable)
    }
}
