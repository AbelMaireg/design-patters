pub mod builder;
pub mod computer;

#[cfg(test)]
mod test {
    use crate::{
        builder::{ComputerBuilder, Director},
        computer::Computer,
    };

    #[test]
    fn test_builder() {
        let mut builder = Computer::default();

        Director::construct_gaming_computer(&mut builder);
        let gaming_computer = builder.build();
        assert_eq!(gaming_computer.cpu, "Intel Core i9");

        Director::construct_office_computer(&mut builder);
        let office_computer = builder.build();
        assert_eq!(office_computer.cpu, "Intel Core i5");
    }
}
