use crate::observer::Event::{Load, Save};
use crate::observer::Publisher;

#[allow(dead_code)]
#[derive(Default)]
pub struct Editor {
    publisher: Publisher,
    data: String,
}

#[allow(dead_code)]
impl Editor {
    pub fn events(&mut self) -> &mut Publisher {
        &mut self.publisher
    }

    pub fn load(&mut self, data: String) {
        self.data = data.clone();
        self.publisher.notify(Load, data);
    }

    pub fn save(&self) {
        self.publisher.notify(Save, self.data.clone());
    }
}
