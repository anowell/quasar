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
            ul id="todo-ul" {
                @for item in &self.items {
                    li class={ "todo-item " (item.complete) } {
                        input type="checkbox" checked?[item.complete]
                        (item.label)
                    }
                }
            }
            input id="message" type="text"
            button { "Add" }
        }).into_string()
    }
}

pub fn init(app: &QuasarApp)  {
    let component = TodoList {
        items: vec![
            TodoItem { label: "Example Task".to_string(), complete: false },
        ]
    };

    let view = app.bind("#todo-list", component);

    // TODO: use view.query("button").on(...)
    // but since we don't patch the DOM yet, the attached element
    // is always destroyed, so we need to attach the event to a nested selector fo now
    view.on_each(EventType::Click, "button".to_string(), |mut evt| {
            let message = evt.binding.query("#message").get("value");
            let item = TodoItem { label: message, complete: false };
            evt.binding.data_mut().items.push(item);
        });

    view.on_each(EventType::Change, ".todo-item input".to_string(), |mut evt| {
        let state = evt.target.checked();
        let mut item_list = evt.binding.data_mut();
        item_list.items[evt.index].complete = state;
    });
}
