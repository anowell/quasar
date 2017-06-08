use quasar::*;
pub use self::counter::CounterData;
mod counter;

impl Component for CounterData {
    fn onload(view: &View<Self>) {
        view.on_each(EventType::Click, "button", |mut evt| {
            evt.binding.data_mut().count += 1;
        });
    }
}