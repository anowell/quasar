#[derive(BartDisplay)]
#[template = "src/todo/bart/todo.html"]
pub struct TodoList {
    pub items: Vec<TodoItem>,
}

pub struct TodoItem {
    pub label: String,
    pub complete: bool,
}

impl TodoItem {
    pub fn new(label: &str) -> TodoItem {
        TodoItem { label: label.to_string(), complete: false }
    }
}