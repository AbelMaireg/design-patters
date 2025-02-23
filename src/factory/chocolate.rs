use super::{Bakery, Cake};

pub struct ChocolateBakery;
pub struct ChocolateCake;

impl Bakery for ChocolateBakery {
    fn create_cake(&self) -> Box<dyn Cake> {
        Box::new(ChocolateCake)
    }
}

impl Cake for ChocolateCake {
    fn desc(&self) -> String {
        "chocolate cake".to_string()
    }
}
