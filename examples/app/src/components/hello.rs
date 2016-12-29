use quasar::*;

#[derive(RustcEncodable)]
struct HelloData {
    name: String,
}

pub fn init(qdom: &mut QuasarDom) {
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

    qdom.render(component, "#hello")
        // .query("#name-field")
        .on(EventType::Input, |evt| {
            evt.component.data.name = evt.target.get("value");
        });
}