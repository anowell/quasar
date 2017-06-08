// mod hello;
mod counter;
// mod cat_list;
mod todo;

use quasar::{QuasarApp, Queryable};
use self::todo::TodoList;
use self::counter::CounterData;

pub fn init(app: &QuasarApp) {
    // hello::init(&app);
    // cat_list::init(&app);
    app.bind("#counter", CounterData::default());
    app.bind("#todo-list", TodoList::default());
}