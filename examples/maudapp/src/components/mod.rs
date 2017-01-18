// mod hello;
mod counter;
// mod cat_list;
mod todo;

use quasar::QuasarApp;

pub fn init(app: &QuasarApp) {
    // hello::init(&app);
    counter::init(&app);
    // cat_list::init(&app);
    todo::init(&app);
}