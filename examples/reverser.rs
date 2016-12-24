extern crate quasar;
extern crate rustc_serialize;

use quasar::{compile_str, Component, EventType};

#[derive(RustcEncodable)]
struct ReverseData {
    message: String,
}


fn main() {
    let mut qdom = quasar::init();

    let my_widget = Component {
        data: ReverseData{
            message: "Hello World".to_owned()
        },
        template: compile_str(r##"
            <p>{{ message }}</p>
            <button>Reverse Message</button>
        "##).expect("failed to compile template")
    };

    let view = qdom.render(my_widget, "Reverser");

    view.on(EventType::Click, |ref mut data| {
        println!("on click called");
        data.message = data.message.chars().rev().collect::<String>();
    });

    println!("End of main");
}