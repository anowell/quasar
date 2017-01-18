use quasar::*;

#[derive(Debug, RustcEncodable)]
struct HelloData {
    name: String,
}

pub fn init(app: &QuasarApp) {
    let component = Component {
        data: HelloData {
            name: "world".to_owned()
        },

        props: vec![],

        template: compile_str(r##"
            <div><input id="name-field" value="world"></div>
            <div>Hello, {{name}}.</div>
        "##).expect("failed to compile hello template")
    };

    app.bind("#hello", component)
        // .query("#name-field")
        .on(EventType::Input, |mut evt| {
            evt.view.data_mut().name = evt.target.get("value");
        });
}