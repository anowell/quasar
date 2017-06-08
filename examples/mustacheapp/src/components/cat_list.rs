use quasar::*;
use helper::RuntimeComponent;

#[derive(Debug, RustcEncodable)]
pub struct CatData { cats: Vec<Cat> }

#[derive(Debug, RustcEncodable)]
pub struct Cat { name: String }

type CatComponent = RuntimeComponent<CatData, ::mustache::Template>;

pub fn init() -> CounterComponent  {
    let cat_names = vec!["Bella", "Tiger", "Chloe", "Shadow", "Luna", "Oreo"];
    let cats = cat_names.iter()
        .map(|c| Cat{ name: c.to_string() })
        .collect();

    RuntimeComponent {
        props: vec![],
        data: CatData {
            cats: cats
        },
        template: compile_str(r##"
            <ul>
            {{#cats}}
                <Cat catname='{{ name }}'></Cat>
            {{/cats}}
            </ul>
        "##).expect("failed to compile cat_list template")
    }

    // TODO: currently lack a way to nest RuntimeComponents
    //   Not sure if that's a good or bad thing yet...
    // RuntimeComponent {
    //     data: (),
    //     props: vec!["catname"],
    //     template: compile_str(r##"
    //         <li>{{ props.catname }}</li>
    //     "##).expect("failed to compile cat_item template")
    // };

}

impl Component for CatComponent {
    fn onload(view: &View<Self>) {
        app.on_each(EventType::Click, "Cat".to_string(), |evt| {
            let catname = evt.target.get("catname");
            println!("MEOW {}", &catname);
        });
    }
}
