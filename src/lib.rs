extern crate webplatform;
extern crate mustache;
extern crate rustc_serialize;

mod events;
mod component;

pub use events::{EventType, Event};
pub use component::Component;

use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::rc::Rc;
use rustc_serialize::Encodable;
use webplatform::{Document, HtmlNode};

pub use mustache::{compile_str, Template};



pub fn init<'a>() -> QuasarDom<'a> {
    QuasarDom {
        document: webplatform::init(),
        components: HashMap::new(),
    }
}

pub struct QuasarDom<'doc> {
    document: Document<'doc>,
    components: HashMap<ViewId, Box<Any>>,
}


impl <'a, 'doc: 'a> QuasarDom<'doc> {
    pub fn render<Data: 'static + Encodable>(&'a mut self, component: Component<Data>, el: &str) -> Views<'a, 'doc, Data> {
        let nodes = self.document.element_query_all(el);
        if nodes.is_empty() {
            panic!("querySelectorAll found no results for {}", &el);
        }

        let view_id = ViewId::new::<Data>(el);
        self.components.insert(
            view_id,
            Box::new(Rc::new(RefCell::new(component))));

        let rc_component = self.component(el);
        let mut views = Vec::new();
        for node in nodes {
            {
                let component = rc_component.borrow();
                node.html_set(&component.render(&node));
            }
            let view = View {
                node: Rc::new(node),
                el: el.to_owned(),
                document: &self.document,
                component: rc_component.clone(),
            };
            views.push(view);
        }
        Views {
            views: views,
            handlers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn view<Data: 'static + Encodable>(&'a self, el: &str) -> View<'a, 'doc, Data>  {
        let view_id = ViewId::new::<Data>(el);
        let entry = self.components.get(&view_id).unwrap();
        let component = self.component(el);
        View {
            node: Rc::new(self.document.element_query(el).unwrap()),
            el: el.to_owned(),
            document: &self.document,
            component: component,
        }
    }

    fn component<Data: 'static + Encodable>(&'a self, el: &str) -> Rc<RefCell<Component<Data>>>  {
        let view_id = ViewId::new::<Data>(el);
        let entry = self.components.get(&view_id).unwrap();
        let component: &Rc<RefCell<Component<Data>>> = entry.downcast_ref().unwrap();
        component.clone()
    }
}

/// A collection of `View`s returned from a query selector
pub struct Views<'a, 'doc: 'a, Data: 'a> {
    views: Vec<View<'a, 'doc, Data>>,
    // Views may have multiple handlers, hence Vec
    // We want interior mutability, hence RefCell
    // A handler may map to multiple views
    handlers: Rc<RefCell<Vec<Box<Fn(Event<Data>) + 'doc>>>>,
}

impl <'a, 'doc: 'a, Data: 'doc + Encodable> Views<'a, 'doc, Data> {
    pub fn on<F>(&self, event: EventType, f: F) where for<'r, 's: 'doc> F: Fn(Event<'r, 's, Data>) + 'doc {
        // Insert the handler into self and return it's index
        let offset = {
            let mut handlers = self.handlers.borrow_mut();
            handlers.push(Box::new(f));
            handlers.len() - 1
        };

        // For each view, setup a unique 'on' handler
        for view in &self.views {
            println!("attaching handler to view: {:?}", &view.node);
            let handlers = self.handlers.clone();
            let rc_component = view.component.clone();
            let node = view.node.clone();
            view.node.on(event.name(), move |evt| {
                let handlers = handlers.clone();
                println!("Event fired on {:?} for target {:?}", &node, evt.target);
                let rendered = {
                    let mut component = rc_component.borrow_mut();
                    let inner_handlers = handlers.borrow();
                    {
                        let event = Event {
                            target: Element { node: &evt.target.expect("Event did not have a target") },
                            data: &mut component.data,
                        };
                        inner_handlers[offset](event);
                    }
                    component.render(&node)
                };
                node.html_set(&rendered);
            });
        }
        println!("{} On handlers registered", self.views.len());
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
    pub fn on<F>(&self, event: EventType, f: F) where for<'r, 's: 'doc> F: Fn(Event<'r, 's, Data>) + 'doc {
        {
            let rc_component = self.component.clone();
            let node = self.node.clone();
            self.node.on(event.name(), move |evt| {
                println!("Event fired on {:?} for target {:?}", &node, evt.target);
                let rendered = {
                    let mut component = rc_component.borrow_mut();
                    {
                        let event = Event {
                            target: Element { node: &evt.target.expect("Event did not have a target") },
                            data: &mut component.data,
                        };
                        f(event);
                    }
                    component.render(&node)
                };
                node.html_set(&rendered);
            });
            println!("On handler registered");
        }
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

#[derive(Debug)]
pub struct Element<'a, 'doc: 'a> {
    node: &'a HtmlNode<'doc>
}

impl <'a, 'doc: 'a> Element<'a, 'doc>{
    pub fn set(&self, prop: &str, value: &str) {
        self.node.prop_set_str(prop, value);
    }

    pub fn get(&self, prop: &str) -> String {
        self.node.prop_get_str(prop)
    }
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
