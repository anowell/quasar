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
        data: TodoList{ items: vec![] },
        template: compile_str(r##"
          <h3>To Do List</h3>
          <ul id="todo-list"></ul>
          <input id="message" type="text">
          <button>Add</button>
        "##).expect("failed to compile todo_list template"),
    };

    let todo_item = NestedComponent {
        data: |data| { data.items },
        template: compile_str(r##"
            <li class="todo-item {{#complete}}complete{{/complete}}">
                <input type="checkbox">
                {{ label }}
            </li>
        "##).expect("failed to compile todo_items template");
    });


    let list_view = app.bind("#todo-list", todo_list);
    list_view.bind(".todo-list", todo_item);


    list_view.bind_map_each(|&mut data| {
        let cc = Collection {
            data: &data.items,
            template: &item_template,
        };
        view.bind_collection(&data.items, ul)
    })


    list_view.query("button")
        .on(EventType::Click, |mut evt| {
            let message = evt.view.query("#message").get("value");
            let item = TodoItem { label: message, complete: false };
            evt.view.data_mut().items.push(item);
        });

    view.query(".todo-item input")
        .on(EventType::Click, |mut evt| {
            let state = evt.target.get("value") == "checked";
            // todo: figure out index
            evt.view.data_mut.items[index].complete = state;
        });

}