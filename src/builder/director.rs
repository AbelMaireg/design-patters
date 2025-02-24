use super::ComputerBuilder;

pub struct Director;

impl Director {
    pub fn construct_gaming_computer(builder: &mut dyn ComputerBuilder) {
        builder.build_cpu("Intel Core i9".to_string());
        builder.build_ram("32GB".to_string());
        builder.build_storage("1TB SSD".to_string());
        builder.build_gpu(Some("NVIDIA RTX 3080".to_string()));
    }

    pub fn construct_office_computer(builder: &mut dyn ComputerBuilder) {
        builder.build_cpu("Intel Core i5".to_string());
        builder.build_ram("16GB".to_string());
        builder.build_storage("512GB SSD".to_string());
        builder.build_gpu(None);
    }
}
