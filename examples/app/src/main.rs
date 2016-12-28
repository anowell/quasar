extern crate quasar;
extern crate rustc_serialize;
mod components;

fn main() {
    let mut qdom = quasar::init();
    println!("Starting...");

    components::hello::init(&mut qdom);
}
