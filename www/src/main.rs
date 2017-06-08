#![feature(plugin)]
#![cfg_attr(feature="with-maud", plugin(maud_macros))]

#[cfg(feature="with-maud")]
extern crate maud;

#[macro_use]
extern crate bart_derive;

extern crate quasar;
extern crate rustc_serialize;

mod code;
mod counter;
mod todo;

use quasar::Queryable;

fn main() {
    let mut app = quasar::init();
    println!("Starting...");

    // unused bindings just to avoid dropping until after app.spin()
    let _code_views = code::init(&mut app);
    app.bind("#counter-bart", counter::bart::CounterData::default());
    app.bind("#todo-bart", todo::bart::TodoList::new());

    app.spin();
}
