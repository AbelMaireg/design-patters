use crate::observer::Event::{Load, Save};
use editor::Editor;

mod editor;
mod observer;

fn main() {
    let mut editor = Editor::default();

    editor
        .events()
        .subscribe(Load, |data| println!("load listener -- {data}"));

    let save_listener = |data| println!("save listener -- {data}");
    editor.events().subscribe(Save, save_listener);

    editor.load("hello".into());
    editor.load("world".into());
    editor.save();

    editor.events().unsubscribe(Save, save_listener);

    editor.save();
}
