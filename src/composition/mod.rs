use crate::command::Command;

#[derive(Default)]
pub struct Composition {
    queue: Vec<Box<dyn Command<Type = String>>>,
}

impl Composition {
    fn add_command(&mut self, command: Box<dyn Command<Type = String>>) {
        self.queue.push(command);
    }

    fn execute(&mut self) {
        for command in self.queue.iter() {
            command.execute();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::button::Button;

    use super::Composition;

    #[test]
    fn test_1() {
        let eraser = Button::new("erase");
        let drawer = Button::new("drawer");
        let painter = Button::new("painter");

        let mut compositor = Composition::default();

        compositor.add_command(Box::new(eraser));
        compositor.add_command(Box::new(drawer));
        compositor.add_command(Box::new(painter));

        compositor.execute();
        assert_eq!(compositor.queue.len(), 0);
    }
}
