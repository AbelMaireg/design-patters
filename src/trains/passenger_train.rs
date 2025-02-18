use super::Train;
use crate::train_station::Mediator;

pub struct PassengerTrain {
    id: String,
}

impl PassengerTrain {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl Train for PassengerTrain {
    fn id(&self) -> &String {
        &self.id
    }

    fn arrive(&mut self, mediator: &mut dyn Mediator) {
        if !mediator.notify_arrival(self.id()) {
            println!("passenger-train :: {} :: platform is BLOCKED!", self.id());
        } else {
            println!("passenger-train :: {} :: platform is OPEN:)", self.id())
        }
    }

    fn depart(&mut self, mediator: &mut dyn Mediator) {
        println!("passenger-train :: {} :: Train has left", self.id());
        mediator.notify_depart(self.id());
    }
}
