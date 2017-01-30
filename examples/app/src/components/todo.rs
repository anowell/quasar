use quasar::*;

#[derive(Debug, RustcEncodable)]
struct TodoItem {
    label: String,
    complete: bool,
}

#[derive(Debug, RustcEncodable)]
struct TodoList {
    items: Vec<TodoItem>,
}

pub fn init(app: &QuasarApp) {
    let todo_list = Component {
        props: vec![],
        data: TodoList{ items: vec![] },
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
    };

    let view = app.bind("#todo-list", todo_list);

    // TODO: use view.query("button").on(...)
    // but since we don't patch the DOM yet, the attached element
    // is always destroyed, so we need to attach the event to a nested selector fo now
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