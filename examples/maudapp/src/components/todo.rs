use quasar::*;

struct TodoItem {
    label: String,
    complete: bool,
}

struct TodoList {
    items: Vec<TodoItem>,
}

impl Renderable for TodoList {
    fn render(&self, _props: Properties) -> String {
        (html! {
            h3 { "To Do List (" (self.items.len()) " items)" }
            ul id="todo-ul" { }
            input id="message" type="text"
            button { "Add" }
        }).into_string()
    }
}

impl Renderable for TodoItem {
    fn render(&self, _props: Properties) -> String {
        (html! {
            li class={ "todo-item " (self.complete) } {
                input type="checkbox"
                (self.label)
            }
        }).into_string()
    }
}

pub fn init(app: &QuasarApp) {
    let component = TodoList {
        items: vec![
            TodoItem { label: "Example Task".to_string(), complete: false },
        ]
    };

    let view = app.bind("#todo-list", component);
    let item_list = view.bind_map_each("#todo-ul", |data| { &data.items });

    view.query("button")
        .on(EventType::Click, |mut evt| {
            let message = evt.view.query("#message").get("value");
            let item = TodoItem { label: message, complete: false };
            evt.view.data_mut().items.push(item);
        });

    // TOOD: shorthand for operation on collection of children views
    // for child in children {
    //     child.query(".todo-item input")
    //         .on(EventType::Click, |mut evt| {
    //             let state = evt.target.get("value") == "checked";
    //             evt.view.data_mut().complete = state;
    //         });
    // }

    // children.query(".todo-item .delete")
    //     .on(EventType::Click, |mut evt| {
    //         evt.view.remove();
    //     });
}
