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

pub fn init(app: &QuasarApp) {
    let component = CounterData {
        count: 0
    };

    app.bind("#counter", component)
        // .query("button")
        .on(EventType::Click, |mut evt| {
            evt.view.data_mut().count += 1;
        });
}