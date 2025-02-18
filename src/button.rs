use crate::command::Command;

pub struct Button {
    label: String,
}

impl Button {
    pub fn new<T: Into<String>>(label: T) -> Self {
        Button {
            label: Into::into(label),
        }
    }
}

impl Command for Button {
    type Type = String;

    fn execute(&self) -> String {
        self.label.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::command::Command;

    use super::Button;

    #[test]
    fn test_1() {
        let b = Button::new("home");
        assert!(b.execute() == "home")
    }
}
