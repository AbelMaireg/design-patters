pub mod chocolate;
pub mod vanilla;

#[allow(unused_imports)]
pub use chocolate::*;

#[allow(unused_imports)]
pub use vanilla::*;

pub trait Bakery {
    fn create_cake(&self) -> Box<dyn Cake>;

    fn bake_cake(&self) -> String {
        let cake = self.create_cake();
        cake.desc()
    }
}

pub trait Cake {
    fn desc(&self) -> String;
}
