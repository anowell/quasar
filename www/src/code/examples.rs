use quasar::*;
use ::std::fmt;

macro_rules! code_file {
    ($lang:expr, $path:expr) => {
        CodeFile {
            filename: $path.rsplit('/').next().unwrap(),
            language: $lang,
            code: include_str!($path),
        }
    }
}

#[derive(Clone)]
pub enum Template { Bart, Maud, Mustache }

impl Template {
    pub fn new(hash: &str) -> Template {
        match hash {
            "maud" => Template::Maud,
            "mustache" => Template::Mustache,
            _ => Template::Bart,
        }
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Template::Bart => write!(f, "bart"),
            Template::Mustache => write!(f, "mustache"),
            Template::Maud => write!(f, "maud"),
        }
    }
}

#[derive(Clone)]
pub enum Example { Counter, Todo }

impl Example {
    fn name(&self) -> String {
        match *self {
            Example::Counter => "counter".to_string(),
            Example::Todo => "todo".to_string(),
        }
    }
}

impl Renderable for Example {
    fn render(&self, _node: &Node, app: &AppContext) -> String {
        let template = app.data::<Template>("template").expect("Failed to get 'template' data");
        println!("RENDERING {} {}", &*template, self.name());
        CodeExample::new(self.clone(), template.clone()).to_string()
    }
}


#[derive(BartDisplay)]
#[template = "src/code/code.html"]
pub struct CodeExample {
    // TODO: figure out how to get this back to an emum
    demo: String,
    template: Template,
    files: Vec<CodeFile>,
}

pub struct CodeFile {
    filename: &'static str,
    code: &'static str,
    language: &'static str,
}

impl CodeExample {
    pub fn new(example: Example, template: Template) -> CodeExample {
        let files = match (&example, &template) {
            (&Example::Counter, &Template::Bart) => vec![
                code_file!("rust", "../counter/bart/mod.rs"),
                code_file!("html", "../counter/bart/counter.html"),
            ],
            (&Example::Counter, &Template::Maud) => vec![
                code_file!("rust", "../counter/maud/mod.rs"),
                code_file!("rust", "../counter/maud/counter.rs"),
            ],
            (&Example::Counter, &Template::Mustache) => vec![
                code_file!("rust", "../counter/mustache/mod.rs"),
            ],
            (&Example::Todo, &Template::Bart) => vec![
                code_file!("rust", "../todo/bart/mod.rs"),
                code_file!("rust", "../todo/bart/todo.rs"),
                code_file!("html", "../todo/bart/todo.html"),
            ],
            (&Example::Todo, &Template::Maud) => vec![
                code_file!("rust", "../todo/maud/mod.rs"),
                code_file!("rust", "../todo/maud/todo.rs"),
            ],
            (&Example::Todo, &Template::Mustache) => vec![
                code_file!("rust", "../todo/mustache/mod.rs"),
            ],
        };

        CodeExample {
            demo: example.name(),
            template: template,
            files: files,
        }
    }
}


