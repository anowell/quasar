extern crate quasar;
extern crate mustache;
extern crate rustc_serialize;

use quasar::{compile_str, Component, EventType, Queryable, HasBind};

#[derive(RustcEncodable)]
struct ReverseData {
    message: String,
}


fn main() {
    let app = quasar::init();

    let my_widget = Component {
        data: ReverseData {
            message: "Hello World".to_owned()
        },
        props: vec!["something"],
        // methods: vec![leftpad, reverse, slugify],
        template: compile_str(r##"
            <p>{{ props.something }}, {{ message }}</p>
            <button>Reverse Message</button>
        "##).expect("failed to compile my_widget template")
    };

    // TODO: this example makes less sense with how things are evolving.
    let view = app.bind("Reverser", my_widget);
    view.on(EventType::Click, |mut evt| {
        println!("on click called");
        let mut data = evt.binding.data_mut();
        data.message = data.message.chars().rev().collect::<String>();
    });

    app.spin();
    println!("End of main");
}