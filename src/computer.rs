use crate::builder::ComputerBuilder;

#[derive(Default)]
pub struct Computer {
    pub cpu: String,
    pub ram: String,
    pub storage: String,
    pub graphics_card: String,
    pub gpu: Option<String>,
}

impl Computer {
    pub fn show_specs(&self) {
        println!("CPU: {}", self.cpu);
        println!("RAM: {}", self.ram);
        println!("Storage: {}", self.storage);
        if let Some(ref gpu) = self.gpu {
            println!("GPU: {}", gpu);
        } else {
            println!("GPU: None");
        }
    }
}

impl ComputerBuilder for Computer {
    fn build_cpu(&mut self, cpu: String) {
        self.cpu = cpu;
    }

    fn build_ram(&mut self, ram: String) {
        self.ram = ram;
    }

    fn build_storage(&mut self, storage: String) {
        self.storage = storage;
    }

    fn build_graphics_card(&mut self, graphics_card: String) {
        self.graphics_card = graphics_card;
    }

    fn build_gpu(&mut self, gpu: Option<String>) {
        self.gpu = gpu;
    }

    fn build(&self) -> Computer {
        Computer {
            cpu: self.cpu.clone(),
            ram: self.ram.clone(),
            storage: self.storage.clone(),
            graphics_card: self.graphics_card.clone(),
            gpu: self.gpu.clone(),
        }
    }
}
