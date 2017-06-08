use quasar::*;
use helper::RuntimeComponent;

#[derive(Debug, Default, RustcEncodable)]
pub struct TodoItem {
    label: String,
    complete: bool,
}

#[derive(Debug, Default, RustcEncodable)]
pub struct TodoList {
    items: Vec<TodoItem>,
}

type TodoComponent = RuntimeComponent<TodoList, ::mustache::Template>;

pub fn init() -> TodoComponent  {
    RuntimeComponent {
        props: vec![],
        data: TodoList::default(),
        template: compile_str(r##"
          <h3>To Do List</h3>
          <ul id="todo-list">
            {{#items}}
                <li class="todo-item {{#complete}}complete{{/complete}}">
                    <input type="checkbox" {{#complete}}checked{{/complete}}>
                    {{ label }}
                </li>
            {{/items}}
          </ul>
          <input id="message" type="text">
          <button>Add</button>
        "##).expect("failed to compile todo_list template"),
    }
}

impl Component for TodoComponent {
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