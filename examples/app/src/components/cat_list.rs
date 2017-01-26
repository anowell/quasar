use quasar::*;

#[derive(Debug, RustcEncodable)]
struct CatData { cats: Vec<Cat> }

#[derive(Debug, RustcEncodable)]
struct Cat { name: String }

pub fn init(app: &QuasarApp) {
    let cat_names = vec!["Bella", "Tiger", "Chloe", "Shadow", "Luna", "Oreo"];
    let cats = cat_names.iter()
        .map(|c| Cat{ name: c.to_string() })
        .collect();

    let cat_list = Component {
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
    };

    let cat_item = Component {
        data: (),
        props: vec!["catname"],
        template: compile_str(r##"
            <li>{{ props.catname }}</li>
        "##).expect("failed to compile cat_item template")
    };

    let view = app.bind(".cat-list", cat_list);

    // TODO: not sure how I want to implement nested components
    //   probably will require much more DOM magic
    // let cat_views = view.bind_each("Cat", |data| data.cats);

    // app.on_each(EventType::Click, "Cat".to_string(), |evt| {
    //     let catname = evt.target.get("catname");
    //     println!("MEOW {}", &catname);
    // });
}