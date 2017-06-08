use quasar::*;

#[derive(Default)]
pub struct CounterData {
    count: u32,
}

impl Renderable for CounterData {
    fn render(&self, _node: &Node, _app: &AppContext) -> String {
        (html! {
            p { "Count: " (self.count) }
            button { "+1" }
        }).into_string()
    }
}

impl Component for CounterData {
    fn onload(view: &View<Self>) {
        view.on_each(EventType::Click, "button", |mut evt| {
            evt.binding.data_mut().count += 1;
        });
    }
}
