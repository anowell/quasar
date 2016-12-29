use quasar::*;

#[derive(RustcEncodable)]
struct CounterData {
    count: u32,
}

pub fn init(qdom: &mut QuasarDom) {
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

    qdom.render(component, "#counter")
        // .query("button")
        .on(EventType::Click, |evt| {
            evt.data.count += 1;
        });
}