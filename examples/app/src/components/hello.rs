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
            <div><input id="name-field" value="{{name}}"></div>
            <div>Hello, {{name}}.</div>
        "##).expect("failed to compile hello template")
    };

    app.bind("#hello", component)
        .on_each(EventType::Input, "#name-field".to_string(), |mut evt| {
            evt.binding.data_mut().name = evt.target.get("value");
        });
}