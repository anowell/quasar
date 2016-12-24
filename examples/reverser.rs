extern crate quasar;
extern crate rustc_serialize;

use quasar::{EventType};

#[derive(RustcEncodable)]
struct ReverseData {
    message: String,
}

fn main() {
    let mut app = quasar::init();

    let data = ReverseData{ message: "Initial Message".to_owned() };
    let view = app.bind("#reverser", data);

    // let view: View<ReverserData> = app.view("#reverser").unwrap()

    // This is basically useless, since we're
    // blocking the event loop until `app` drops
    view.update( |ref mut data| {
        println!("{}", data.message);
        data.message = "Hello World".to_owned();
    });

    // TODO: subquery via view.query("button").on...
    view.on(EventType::Click, |ref mut data| {
        data.message = data.message.chars().rev().collect::<String>();
    });

    println!("End of main");
}
