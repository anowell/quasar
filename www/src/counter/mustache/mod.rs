use quasar::*;

#[derive(Debug, Default, RustcEncodable)]
struct CounterData {
    count: u32,
}

type CounterComponent = RuntimeComponent<CounterData, ::mustache::Template>;
pub fn init() -> CounterComponent {
    let component = RuntimeComponent {
        data: CounterData { count: 0 },
        props: vec![],
        template: compile_str(r##"
            <p>Count: {{count}}</p>
            <button>+1</button>
        "##).expect("failed to compile counter template")
    };
}

impl Component for CounterComponent {
    fn onload(view: &View<Self>) {
        view.on_each(EventType::Click, "button", |mut evt| {
            evt.binding.data_mut().count += 1;
        });
    }
}