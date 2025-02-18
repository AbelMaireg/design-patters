use std::collections::{HashMap, VecDeque};

use crate::trains::Train;

pub trait Mediator {
    fn notify_arrival(&mut self, id: &str) -> bool;
    fn notify_depart(&mut self, id: &str);
}

#[derive(Default)]
pub struct TrainStation {
    trains: HashMap<String, Box<dyn Train>>,
    trains_queue: VecDeque<String>,
    train_on_platform: Option<String>,
}

impl Mediator for TrainStation {
    fn notify_arrival(&mut self, train_id: &str) -> bool {
        if self.train_on_platform.is_some() {
            self.trains_queue.push_back(train_id.into());

            false
        } else {
            self.train_on_platform.replace(train_id.into());
            true
        }
    }

    fn notify_depart(&mut self, train_id: &str) {
        if Some(train_id.into()) == self.train_on_platform {
            self.train_on_platform = None;

            if let Some(train) = self.trains_queue.pop_front() {
                let mut next_train = self.trains.remove(&train).unwrap();
                next_train.arrive(self);
                self.trains.insert(next_train.id().clone(), next_train);
                self.train_on_platform = Some(train);
            }
        }
    }
}

impl TrainStation {
    pub fn accept(&mut self, mut train: impl Train + 'static) {
        if self.trains.contains_key(train.id()) {
            println!("{} has already arrived", train.id());
            return;
        }

        train.arrive(self);
        self.trains.insert(train.id().clone(), Box::new(train));
    }

    pub fn depart(&mut self, name: &'static str) {
        let train = self.trains.remove(name);
        if let Some(mut train) = train {
            train.depart(self);
        } else {
            println!("'{}' is not on the station!", name);
        }
    }
}
