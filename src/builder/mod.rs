use crate::computer::Computer;
mod director;

pub use director::Director;

pub trait ComputerBuilder {
    fn build_cpu(&mut self, cpu: String);
    fn build_ram(&mut self, ram: String);
    fn build_storage(&mut self, storage: String);
    fn build_graphics_card(&mut self, graphics_card: String);
    fn build_gpu(&mut self, gpu: Option<String>);
    fn build(&self) -> Computer;
}
