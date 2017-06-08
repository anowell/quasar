use quasar::*;
use helper::RuntimeComponent;

#[derive(Debug, RustcEncodable)]
pub struct HelloData {
    name: String,
}

type HelloComponent = RuntimeComponent<HelloData, ::mustache::Template>;

pub fn init() -> HelloComponent  {
    RuntimeComponent {
        data: HelloData {
            name: "world".to_owned()
        },

        props: vec![],

        template: compile_str(r##"
            <div><input id="name-field" value="{{name}}"></div>
            <div>Hello, {{name}}.</div>
        "##).expect("failed to compile hello template")
    }
}


impl Component for HelloComponent {
    fn onload(view: &View<Self>) {
        view.on_each(EventType::Input, "#name-field", |mut evt| {
            evt.binding.data_mut().name = evt.target.get("value");
        });
    }
}
