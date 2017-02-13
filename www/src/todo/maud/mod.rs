use quasar::*;
use self::todo::{TodoItem, TodoList};
mod todo;

pub fn init(app: &QuasarApp) {
    let todo_list = TodoList {
        items: vec![
            TodoItem::new("Blog about Quasar"),
        ]
    };

    let view = app.bind("#todo-bart", todo_list);

    view.query("button").expect("missing todo list button")
        .on(EventType::Click, |mut evt| {
            match evt.app.query("#message") {
                Some(node) => {
                    let item = TodoItem::new(&node.get("value"));
                    evt.binding.data_mut().items.push(item);
                }
                None => println!("Query #message returned nothing.")
            }
        });

    view.on_each(EventType::Change, ".todo-item input", |mut evt| {
        let state = evt.target.checked();
        let mut item_list = evt.binding.data_mut();
        item_list.items[evt.index].complete = state;
    });

}