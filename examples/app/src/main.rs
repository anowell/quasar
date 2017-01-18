extern crate quasar;
extern crate rustc_serialize;
mod components;

fn main() {
    let app = quasar::init();
    println!("Starting...");

    components::init(&app);
    app.spin();
}
