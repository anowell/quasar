#![feature(plugin)]
#![plugin(maud_macros)]

extern crate quasar;
extern crate maud;

mod components;

fn main() {
    let mut qdom = quasar::init();
    println!("Starting...");

    //components::hello::init(&mut qdom);
    components::counter::init(&mut qdom);
    //components::cat_list::init(&mut qdom);
}
