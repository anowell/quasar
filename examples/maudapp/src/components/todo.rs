use quasar::*;

#[derive(Default)]
pub struct TodoItem {
    label: String,
    complete: bool,
}

#[derive(Default)]
pub struct TodoList {
    items: Vec<TodoItem>,
}

impl Renderable for TodoList {
    fn render(&self, _node: &Node, _app: &AppContext) -> String {
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

impl Component for TodoList {
    fn onload(view: &View<Self>) {
        view.on_each(EventType::Click, "button", |mut evt| {
            match evt.binding.query("#message") {
                Some(node) => {
                    let item = TodoItem { label: node.get("value"), complete: false };
                    evt.binding.data_mut().items.push(item);
                }
                None => println!("Query for #message returned nothing.")
            }
        });

        view.on_each(EventType::Change, ".todo-item input", |mut evt| {
            let state = evt.target.checked();
            let mut item_list = evt.binding.data_mut();
            item_list.items[evt.index].complete = state;
        });
    }
}
