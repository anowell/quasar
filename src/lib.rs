extern crate webplatform;
extern crate mustache;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::rc::Rc;
use rustc_serialize::Encodable;
use webplatform::{Document, HtmlNode};

pub use mustache::{compile_str, Template};

pub struct Component<Data> {
    pub template: Template,
    pub data: Data,
}

impl <Data: Encodable> Component<Data> {
    fn render(&self) -> String {
        let mut output = Vec::new();
        self.template.render(&mut output, &self.data).expect("failed to render component");
        String::from_utf8_lossy(&output).into_owned()
    }
}

pub enum EventType {
    Click,
}

impl EventType {
    fn name(&self) -> &'static str {
        match *self {
            EventType::Click => "click",
        }
    }
}

pub struct View<'a, 'doc: 'a, Data: 'a> {
    // document and el are redundant of node,
    // but needed for nested queries
    document: &'a Document<'doc>,
    el: String,

    node: Rc<HtmlNode<'doc>>,
    component: Rc<RefCell<Component<Data>>>,
}

impl <'a, 'doc: 'a, Data: 'doc + Encodable> View<'a, 'doc, Data> {
    pub fn update<F>(&self, f: F) where F: Fn(&mut Data) {
        println!("Update called {:?}", self.node);
        {
            let mut component = self.component.borrow_mut();
            f(&mut component.data);
        }
        self.repaint()
    }

    pub fn on<F: 'doc>(&self, event: EventType, f: F) where F: Fn(&mut Data) {
        println!("On called {:?}", self.node);
        {
            let mut rc_component = self.component.clone();
            let node = self.node.clone();
            self.node.on(event.name(), move |evt| {
                println!("Event fired for target {:?}", evt.target);
                let rendered = {
                    let mut component = rc_component.borrow_mut();
                    f(&mut component.data);
                    component.render()
                };
                node.html_set(&rendered);
            });
            println!("On handler registered");
        }
    }

    fn repaint(&self) {
        println!("Repaint called {:?}", self.node);
        let component = self.component.borrow();
        let rendered = component.render();
        self.node.html_set(&rendered);
    }

}

#[derive(Hash, Eq, PartialEq)]
pub struct ViewId {
    tid: TypeId,
    selector: String,
}

impl ViewId {
    fn new<Data: 'static + Encodable>(el: &str) -> ViewId {
        ViewId {
            tid: TypeId::of::<Data>(),
            selector: el.to_owned(),
        }
    }
}

pub fn init<'a>() -> QuasarDom<'a> {
    QuasarDom {
        document: webplatform::init(),
        views: HashMap::new(),
    }
}

pub struct QuasarDom<'doc> {
    document: Document<'doc>,
    views: HashMap<ViewId, Box<Any>>,
}


impl <'a, 'doc: 'a> QuasarDom<'doc> {
    pub fn render<Data: 'static + Encodable>(&'a mut self, component: Component<Data>, el: &str) -> View<'a, 'doc, Data> {
        let node = self.document.element_query(el).expect("failed to query element");
        node.html_set(&component.render());

        let view_id = ViewId::new::<Data>(el);
        self.views.insert(
            view_id,
            Box::new(Rc::new(RefCell::new(component))));

        self.view(el)
    }

    pub fn view<Data: 'static + Encodable>(&'a self, el: &str) -> View<'a, 'doc, Data>  {
        let view_id = ViewId::new::<Data>(el);
        let entry = self.views.get(&view_id).unwrap();
        let component: &Rc<RefCell<Component<Data>>> = entry.downcast_ref().unwrap();
        View {
            node: Rc::new(self.document.element_query(el).unwrap()),
            el: el.to_owned(),
            document: &self.document,
            component: component.clone(),
        }
    }

    // pub fn query(&'a self, el: &str) -> Node<'doc>  {
    //     let node = self.document.element_query(el).unwrap();
    //     Node {
    //         node: node,
    //         el: el.to_owned(),
    //     }
    // }

    // pub fn on
}


impl <'doc> Drop for QuasarDom<'doc> {
    fn drop(&mut self) {
        webplatform::spin();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        unimplemented!()
    }
}
