use quasar::*;

struct CounterData {
    count: u32,
}

impl Renderable for CounterData {
    fn render(&self, _props: Properties) -> String {
        (html! {
            p { "Count: " (self.count) }
            button { "+1" }
        }).into_string()
    }
}

pub fn init(qdom: &mut QuasarDom) {
    let component = CounterData {
        count: 0
    };

    qdom.render(component, "#counter")
        // .query("button")
        .on(EventType::Click, |evt| {
            evt.component.count += 1;
        });
}