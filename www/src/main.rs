#![feature(plugin)]
#![plugin(maud_macros)]

extern crate quasar;
extern crate maud;
extern crate rustc_serialize;
#[macro_use] extern crate bart_derive;

mod code;
mod counter;
mod todo;

fn main() {
    let mut app = quasar::init();
    println!("Starting...");

    code::init(&mut app);
    counter::bart::init(&app);
    todo::bart::init(&app);
    app.spin();
}
