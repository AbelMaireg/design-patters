fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
struct Editor {
    content: String,
}

#[allow(dead_code)]
impl Editor {
    fn create_state(&self) -> EditorState {
        EditorState::new(self.content.clone())
    }

    fn restore(&mut self, state: Option<EditorState>) {
        if state.is_some() {
            self.content = state.unwrap().content;
        }
    }
}

#[allow(dead_code)]
struct EditorState {
    content: String,
}

#[allow(dead_code)]
impl EditorState {
    fn new(content: String) -> Self {
        EditorState { content }
    }
}

#[allow(dead_code)]
struct History {
    list: Vec<EditorState>,
}

#[allow(dead_code)]
impl History {
    fn push(&mut self, state: EditorState) {
        self.list.push(state);
    }

    fn pop(&mut self) -> Option<EditorState> {
        self.list.pop()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut editor = Editor {
            content: "".to_string(),
        };
        let mut history = History { list: vec![] };

        editor.content = "a".to_string();
        history.push(editor.create_state());

        editor.content = "b".to_string();
        history.push(editor.create_state());

        editor.content = "c".to_string();

        editor.restore(history.pop());
        assert_eq!(editor.content, "b".to_string());

        editor.restore(history.pop());
        assert_eq!(editor.content, "a".to_string());
    }
}
