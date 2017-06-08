use quasar::*;

#[derive(Default, BartDisplay)]
#[template = "src/counter/bart/counter.html"]
pub struct CounterData { count: u32 }

impl Component for CounterData {
    fn onload(view: &View<Self>) {
        view.on_each(EventType::Click, "button", |mut evt| {
            evt.binding.data_mut().count += 1;
        });
    }
}