use quasar::*;

#[derive(Default)]
pub struct CounterData {
    pub count: u32,
}

impl Renderable for CounterData {
    fn render(&self, _node: &Node, _app: &AppContext) -> String {
        (html! {
            p { "Count: " (self.count) }
            button { "+1" }
        }).into_string()
    }
}