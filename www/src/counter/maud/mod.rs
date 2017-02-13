use quasar::*;
use self::counter::CounterData;
mod counter;

pub fn init(app: &QuasarApp) {
    let component = CounterData {
        count: 0
    };

    app.bind("#counter", component)
        .on_each(EventType::Click, "button", |mut evt| {
            evt.binding.data_mut().count += 1;
        });
}