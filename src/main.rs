fn main() {
    let mut data = DataSource::new();
    data.set(vec![45, 83, 98]);

    let sheet = SpreadSheet {};
    let chart = Chart {};

    data.subject.add(Box::new(sheet));
    data.subject.add(Box::new(chart));

    data.set(vec![1, 2, 3]);
}

struct DataSource {
    content: Vec<i32>,
    subject: Subject,
}

#[allow(dead_code)]
impl DataSource {
    fn new() -> Self {
        DataSource {
            content: vec![],
            subject: Subject::new(),
        }
    }

    fn get(&self) -> &Vec<i32> {
        &self.content
    }

    fn set(&mut self, content: Vec<i32>) {
        self.content = content;
        self.subject.notify(&self.content)
    }
}

#[allow(dead_code)]
trait Observer {
    fn update(&self, data: &[i32]);
}

#[allow(dead_code)]
struct SpreadSheet {}

#[allow(dead_code)]
struct Chart {}

#[allow(dead_code, clippy::extra_unused_lifetimes)]
impl Observer for SpreadSheet {
    fn update(&self, data: &[i32]) {
        println!("spreadsheet updated! {:?}", data)
    }
}

#[allow(dead_code, clippy::extra_unused_lifetimes)]
impl Observer for Chart {
    fn update(&self, data: &[i32]) {
        println!("chart updated! {:?}", data)
    }
}

#[allow(dead_code)]
struct Subject {
    observers: Vec<Box<dyn Observer>>,
}

#[allow(dead_code)]
impl Subject {
    fn new() -> Self {
        Subject { observers: vec![] }
    }

    fn add(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn remove(&mut self, observer_idx: usize) {
        if observer_idx < self.observers.len() {
            self.observers.remove(observer_idx);
        }
    }

    fn notify(&self, data: &[i32]) {
        for observer in &self.observers {
            observer.update(data)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Chart, DataSource, Observer, SpreadSheet};

    #[test]
    fn test() {
        let mut data = DataSource::new();
        data.set(vec![45, 83, 98]);

        let sheet = SpreadSheet {};
        let chart = Chart {};

        data.subject.add(Box::new(sheet));
        data.subject.add(Box::new(chart));

        data.set(vec![1, 2, 3]);
    }
}
