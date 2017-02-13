use quasar::*;

#[derive(BartDisplay)]
#[template = "src/counter/bart/counter.html"]
struct CounterData { count: u32 }

pub fn init(app: &QuasarApp) {
    let component = CounterData { count: 0 };

    app.bind("#counter-bart", component)
        .query("button").unwrap()
        .on(EventType::Click, |mut evt| {
            evt.binding.data_mut().count += 1;
        });
}