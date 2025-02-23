pub mod factory;

#[cfg(test)]
mod test {
    use crate::factory::*;

    #[test]
    fn test_chocolate_factory() {
        let bakery = ChocolateBakery;
        assert_eq!(bakery.bake_cake(), "chocolate cake");
    }

    #[test]
    fn test_vanilla_factory() {
        let bakery = VanillaBakery;
        assert_eq!(bakery.bake_cake(), "vanilla cake")
    }
}
