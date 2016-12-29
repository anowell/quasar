use quasar::*;

#[derive(RustcEncodable)]
struct CatData { cats: Vec<Cat> }

#[derive(RustcEncodable)]
struct Cat { name: String }

pub fn init(qdom: &mut QuasarDom) {
    let cats = vec!["Bella", "Tiger", "Chloe", "Shadow", "Luna", "Oreo"];

    let cat_list = Component {
        data: CatData {
            cats: cats.iter().map(|c| Cat{ name: c.to_string() }).collect()
        },

        props: vec![],

        template: compile_str(r##"
            <ul>
            {{#cats}}
                <Cat catname='{{ name }}'></Cat>
            {{/cats}}
            </ul>
        "##).expect("failed to compile cat_list template")
    };

    let cat_item = Component {
        data: Cat { name: "what?".to_string() },
        props: vec!["catname"],
        template: compile_str(r##"
            <li>{{ props.catname }}</li>
        "##).expect("failed to compile cat_item template")
    };

    qdom.render(cat_list, ".cat-list");
    // TODO: allow chaining renders for nested
    qdom.render(cat_item, "Cat");
}