#![feature(plugin)]
#![plugin(maud_macros)]

extern crate quasar;
extern crate maud;

mod components;

fn main() {
    let app = quasar::init();
    println!("Starting...");

    //components::hello::init(&app);
    components::counter::init(&app);
    //components::cat_list::init(&app);
    app.spin();
}
