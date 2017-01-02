extern crate quasar;
extern crate rustc_serialize;
mod components;

fn main() {
    let app = quasar::init();
    println!("Starting...");

    components::hello::init(&app);
    components::counter::init(&app);
    components::cat_list::init(&app);
    app.spin();
}
