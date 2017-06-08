extern crate quasar;
extern crate mustache;
extern crate rustc_serialize;
mod components;
mod helper;
pub use helper::RuntimeComponent;

fn main() {
    let app = quasar::init();
    println!("Starting...");

    components::init(&app);
    app.spin();
}
