extern crate webplatform;
extern crate rustc_serialize;
extern crate owning_ref;

#[cfg(feature = "mustache")]
extern crate mustache;

mod events;
mod components;

pub use events::{EventType, Event};
pub use components::{Component, Properties, Renderable};
pub use rustc_serialize::json::Json;

use owning_ref::{RefRef, RefMutRef};
use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
use webplatform::{Document, HtmlNode};

#[cfg(feature = "mustache")]
pub use mustache::compile_str;


pub fn init<'a, 'doc: 'a>() -> QuasarApp<'a> {
    // TODO: set_main_loop to a function that processes the render queue

    QuasarApp {
        document: Rc::new(webplatform::init()),
        components: Rc::new(RefCell::new(HashMap::new())),
        state: Rc::new(RefCell::new(HashMap::new())),
        render_queue: Rc::new(RefCell::new(Vec::new())),
    }
}

// TODO: revisit cloning of main app..
// it feels very strange that QuasarApp is basically an `Rc` type
// but it's non-trivial to pass around &QuasarApp since events need access
// and almost certainly outlive the app instance if not for all the Rc members
#[derive(Clone)]
pub struct QuasarApp<'doc> {
    document: Rc<Document<'doc>>,
    components: Rc<RefCell<HashMap<TypedKey, Box<Any>>>>,
    state: Rc<RefCell<HashMap<TypedKey, Box<Any>>>>,
    render_queue: Rc<RefCell<Vec<(TypedKey, Rc<HtmlNode<'doc>>)>>>,
}

type DataRef<'a, T> = RefRef<'a, HashMap<TypedKey, Box<Any>>, T>;
type DataMutRef<'a, T> = RefMutRef<'a, HashMap<TypedKey, Box<Any>>, &'a mut T>;

impl <'doc> QuasarApp<'doc> {
    pub fn bind<R: 'static + Renderable>(&self, component: R, el: &str) -> Views<'doc, R> {
        let nodes = self.document.element_query_all(el);
        if nodes.is_empty() {
            panic!("querySelectorAll found no results for {}", &el);
        }

        {
            let view_id = TypedKey::new::<R>(el);
            let mut components = self.components.borrow_mut();
            components.insert(
                view_id,
                Box::new(Rc::new(RefCell::new(component)))
            );
        }

        let rc_component = self.component(el);
        let mut views = Vec::new();
        for node in nodes {
            {
                let component = rc_component.borrow();
                let props = lookup_props(&node, R::props(&component));
                node.html_set(&component.render(props));
            }
            let view = View {
                app: self.clone(),
                node: Rc::new(node),
                el: el.to_owned(),
                component: rc_component.clone(),
            };
            views.push(view);
        }
        Views {
            app: self.clone(),
            views: Rc::new(views),
            handlers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn view<R: 'static + Renderable>(&self, el: &str) -> View<'doc, R>  {
        let component = self.component(el);
        View {
            app: self.clone(),
            node: Rc::new(self.document.element_query(el).unwrap()),
            el: el.to_owned(),
            component: component,
        }
    }

    fn component<R: 'static + Renderable>(&self, el: &str) -> Rc<RefCell<R>>  {
        let view_id = TypedKey::new::<R>(el);
        let components = self.components.borrow();
        let entry = components.get(&view_id).unwrap();
        let component: &Rc<RefCell<R>> = entry.downcast_ref().unwrap();
        component.clone()
    }

    pub fn data<T: 'static>(&self, key: &str) -> DataRef<T> {
        let data_id = TypedKey::new::<T>(key);
        RefRef::new(self.state.borrow()).map(|state| {
            let entry = state.get(&data_id).unwrap();
            entry.downcast_ref().unwrap()
        })
    }

    pub fn data_mut<T: 'static>(&self, key: &str) -> DataMutRef<T> {
        // TODO: look up observer views, and add enqueue for re-render
        let data_id = TypedKey::new::<T>(key);
        RefMutRef::new(self.state.borrow_mut()).map(|state| {
            // TODO: Look into getting an `OwnedMutRef` that supports `map_mut`
            let state = state as *const HashMap<_, _> as *mut HashMap<TypedKey, Box<Any>>;
            let mut state = unsafe { &mut *state };
            let mut entry = state.get_mut(&data_id).unwrap();
            entry.downcast_mut().unwrap()
        })
    }


    fn enqueue_render<R: 'static + Renderable>(&self, view: &View<'doc, R>) {
        let mut queue = self.render_queue.borrow_mut();
        let view_id = TypedKey::new::<R>(&view.el);
        queue.push((view_id, view.node.clone()));
    }

    pub fn spin(self) {
        webplatform::spin();
    }
}

/// A collection of `View`s returned from a query selector
pub struct Views<'doc, R> {
    app: QuasarApp<'doc>,
    pub views: Rc<Vec<View<'doc, R>>>,
    // Views may have multiple handlers, hence Vec
    // We want interior mutability, hence RefCell
    // A handler may map to multiple views
    handlers: Rc<RefCell<Vec<Box<Fn(Event<R>) + 'doc>>>>,
}

impl <'doc, R: Renderable + 'doc> Views<'doc, R> {
    pub fn on<F>(&self, event: EventType, f: F) where F: Fn(Event<R>) + 'doc {
        // Insert the handler into self and return it's index
        let offset = {
            let mut handlers = self.handlers.borrow_mut();
            handlers.push(Box::new(f));
            handlers.len() - 1
        };


        // For each view, setup a unique 'on' handler
        for v in self.views.iter() {
            println!("attaching handler to view: {:?}", &v.node);
            let handlers = self.handlers.clone();
            let app = v.app.clone();
            let el = v.el.clone();
            let node = v.node.clone();
            let component = v.component.clone();
            let views = self.views.clone();

            v.node.on(event.name(), move |evt| {
                let handlers = handlers.clone();

                let view = View {
                    app: app.clone(),
                    el: el.clone(),
                    node: node.clone(),
                    component: component.clone(),
                };

                // Process the event with the target and originating view
                println!("Event fired on {:?} for target {:?}", &view.node, evt.target);
                {
                    let event = Event {
                        app: app.clone(),
                        target: Element {
                            node: evt.target.expect("Event did not have a target") ,
                        },
                        view: view,
                    };
                    let inner_handlers = handlers.borrow();
                    inner_handlers[offset](event);
                }

                // Re-render all the views for this component instance
                // TODO: remove this section
                // instead, bind should add all these views as observers of this component data
                // and let `data_mut` schedule re-render which could be handled here or in tick loop
                for v in views.iter() {
                    let node = v.node.clone();
                    let component = v.component.clone();
                    let rendered = {
                        let mut component = component.borrow_mut();
                        let props = lookup_props(&node, component.props());
                        component.render(props)
                    };
                    node.html_set(&rendered);
                }
            });
        }
        println!("{} On handlers registered", self.views.len());
    }
}

fn lookup_props<'doc>(node: &HtmlNode<'doc>, keys: &[&'static str]) -> Properties {
    let mut props = Properties::new();
    for prop in keys {
        let mut val = node.prop_get_str(prop);
        if val.is_empty() {
            val = node.attr_get_str(prop);
        }
        props.insert(prop, val);
    }
    props
}

pub struct View<'doc, R> {
    app: QuasarApp<'doc>,
    // Fully qualified query selector - append to any parent selectors used to get to this view
    el: String,
    node: Rc<HtmlNode<'doc>>,
    component: Rc<RefCell<R>>,
}

impl <'doc, R: 'static + Renderable> View<'doc, R> {
    pub fn on<F>(&self, event: EventType, f: F) where F: Fn(Event<R>) + 'doc {
        {
            let app = self.app.clone();
            let el = self.el.clone();
            let component = self.component.clone();
            let node = self.node.clone();
            self.node.on(event.name(), move |evt| {

                // FIXME: why so much extra cloning to avoid E0507?
                let view = View {
                    app: app.clone(),
                    el: el.clone(),
                    node: node.clone(),
                    component: component.clone(),
                };

                println!("Event fired on {:?} for target {:?}", &view.node, evt.target);
                let rendered = {
                    {
                        let target_node = evt.target.expect("Event did not have a target");
                        let event = Event {
                            app: app.clone(),
                            target: Element { node: target_node },
                            view: view,
                        };
                        f(event);
                    }
                    let component = component.borrow();
                    let props = lookup_props(&node, component.props());
                    component.render(props)
                };
                node.html_set(&rendered);
            });
            println!("On handler registered");
        }
    }

    pub fn data(&self) -> Ref<R> {
        self.component.borrow()
    }

    pub fn data_mut(&mut self) -> RefMut<R> {
        // Before handing back mutable the mutable component,
        // enqueue rendering of the original view that owns this data
        self.app.enqueue_render(&self);
        self.component.borrow_mut()
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct TypedKey {
    tid: TypeId,
    key: String,
}

impl TypedKey {
    fn new<R: 'static>(key: &str) -> TypedKey {
        TypedKey {
            tid: TypeId::of::<R>(),
            key: key.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct Element<'doc> {
    node: HtmlNode<'doc>
}

impl <'doc> Element<'doc>{
    pub fn set(&self, prop: &str, value: &str) {
        self.node.prop_set_str(prop, value);
    }

    pub fn get(&self, prop: &str) -> String {
        self.node.prop_get_str(prop)
    }
}
