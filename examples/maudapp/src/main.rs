#![feature(plugin)]
#![plugin(maud_macros)]

extern crate quasar;
extern crate maud;

mod components;

fn main() {
    let app = quasar::init();
    println!("Starting...");

    components::init(&app);
    println!("Spinning...");
    app.spin();
    println!("End of main...");
}
