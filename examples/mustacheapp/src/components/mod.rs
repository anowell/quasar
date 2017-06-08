mod hello;
mod counter;
// mod cat_list;
mod todo;

use quasar::{QuasarApp, Queryable};

pub fn init(app: &QuasarApp) {
    app.bind("#hello", hello::init());
    app.bind("#counter", counter::init());

    // app.bind(".cat-list", cat_list::init());

    app.bind("#todo-list", todo::init());

}