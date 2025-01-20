use std::collections::HashMap;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Event {
    Load,
    Save,
}

#[allow(dead_code)]
pub type Subscriber = fn(data: String);

#[allow(dead_code)]
#[derive(Default)]
pub struct Publisher {
    events: HashMap<Event, Vec<Subscriber>>,
}

#[allow(dead_code)]
impl Publisher {
    pub fn subscribe(&mut self, event: Event, listener: Subscriber) {
        self.events.entry(event.clone()).or_default();
        self.events.get_mut(&event).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event: Event, listener: Subscriber) {
        self.events
            .get_mut(&event)
            .unwrap()
            .retain(|&x| x != listener);
    }

    pub fn notify(&self, event: Event, data: String) {
        for listener in self.events.get(&event).unwrap() {
            listener(data.clone());
        }
    }
}
