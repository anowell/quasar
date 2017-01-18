use quasar::*;

#[derive(Debug, RustcEncodable)]
struct CounterData {
    count: u32,
}

pub fn init(app: &QuasarApp) {
    let component = Component {
        data: CounterData {
            count: 0
        },

        props: vec![],

        template: compile_str(r##"
            <p>Count: {{count}}</p>
            <button>+1</button>
        "##).expect("failed to compile counter template")
    };

    let view = app.bind("#counter", component);

    let element = view.query("button");
    element.on(EventType::Click, |mut evt| {
        evt.view.data_mut().count += 1;
    });
}