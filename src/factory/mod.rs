pub mod art_deco;
pub mod modern;

#[allow(unused_imports)]
pub use art_deco::*;

#[allow(unused_imports)]
pub use modern::*;

pub trait FurnitureFactory {
    fn create_chair(&self) -> Box<dyn ChairFactory>;
    fn create_table(&self) -> Box<dyn TableFactory>;
}

pub trait ChairFactory {
    fn operation(&self) -> String;
}

pub trait TableFactory {
    fn operation(&self) -> String;
}
