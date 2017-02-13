use quasar::*;
mod examples;
use self::examples::{Example, Template};

pub fn init(app: &mut QuasarApp) {
    app.data_set("template", Template::new("bart"));

    app.bind("#counter-code", Example::Counter);
    app.bind("#todo-code", Example::Todo);

    for template_selector in app.query_all("input[name=\"template\"]") {
        template_selector.on(EventType::Change, |mut evt| {
            let template = Template::new(&evt.target.get_attr("data-template"));
            println!("template selector change: {}", &template);
            evt.app.data_mut("template").map(|mut t| *t = template);
        });
    }
}

