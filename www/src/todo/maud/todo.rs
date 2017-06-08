use quasar::*;

pub struct TodoItem {
    pub label: String,
    pub complete: bool,
}

pub struct TodoList {
    pub items: Vec<TodoItem>,
}

impl TodoList{
    fn new() -> TodoList {
        TodoList {
            items: vec![TodoItem::new("Blog about Quasar")]
        }
    }
}

impl TodoItem {
    pub fn new(label: &str) -> TodoItem {
        TodoItem { label: label.to_string(), complete: false }
    }
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

