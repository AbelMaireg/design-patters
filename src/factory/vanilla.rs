use super::{Bakery, Cake};

pub struct VanillaBakery;
pub struct VanillaCake;

impl Bakery for VanillaBakery {
    fn create_cake(&self) -> Box<dyn Cake> {
        Box::new(VanillaCake)
    }
}

impl Cake for VanillaCake {
    fn desc(&self) -> String {
        "vanilla cake".to_string()
    }
}
