use state::{AppState, Binding, TypedKey};
use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
use std::marker::PhantomData;
use webplatform::{self, Document, HtmlNode};

use {Renderable, Properties, Event, EventType};

/// The main app object instantiated by calling `quasar::init()`
pub struct QuasarApp<'doc> {
    app: Rc<AppState<'doc>>,
    document: Rc<Document<'doc>>,
}

pub struct Node<'doc> {
    app: Rc<AppState<'doc>>,
    node: HtmlNode<'doc>,
}

pub struct View<'doc, R> {
    app: Rc<AppState<'doc>>,
    node: Rc<HtmlNode<'doc>>,
    key: String,
    binding: Rc<RefCell<Binding<'doc>>>,
    phantom: PhantomData<R>, // TODO: generic marker SingleBind or Multibind to indicate if we can iterate
}


pub trait Queryable<'doc> {
    type Q: Queryable<'doc>;

    fn query(&self, el: &str) -> Option<Self::Q>;
    // fn query_all(&self, el: &str) -> Vec<Self>

    fn bind<R>(&self, el: &str, component: R) -> View<'doc, R> where R: 'static + Renderable;
    // fn bind_each(&self, el: &str, component: Vec<R>) -> BindEachNode<'doc, R>;
}

pub trait HasBind<'doc> {
    type R: Renderable;

    fn data(&self) -> Ref<Self::R>;
    fn data_mut(&mut self) -> RefMut<Self::R>;
}

impl<'doc> Queryable<'doc> for QuasarApp<'doc> {
    type Q = Node<'doc>;
    fn query(&self, el: &str) -> Option<Node<'doc>> {
        self.document.element_query(el).map(|node| {
            Node {
                app: self.app.clone(),
                node: node,
            }
        })
    }

    fn bind<R: 'static + Renderable>(&self, el: &str, component: R) -> View<'doc, R> {
        let node = self.document.element_query(el).expect("querySelector found no results");

        let props = lookup_props(&node, component.props());
        node.html_patch(&component.render(props));

        let rc_node = Rc::new(node);
        let key = "TODO: generate a unique key";
        let binding = self.app.insert_binding(key, component, rc_node.clone());

        View {
            app: self.app.clone(),
            key: key.to_string(),
            node: rc_node,
            binding: binding,
            phantom: PhantomData,
        }
    }
}

impl<'doc> Queryable<'doc> for Node<'doc> {
    type Q = Self;

    fn query(&self, el: &str) -> Option<Self::Q> {
        self.node.element_query(el).map(|node| {
            Node {
                app: self.app.clone(),
                node: node,
            }
        })
    }

    fn bind<RR>(&self, el: &str, component: RR) -> View<'doc, RR>
        where RR: 'static + Renderable
    {
        let node = self.node.element_query(el).expect("querySelector found no results");
        let props = lookup_props(&node, component.props());
        node.html_patch(&component.render(props));

        let rc_node = Rc::new(node);
        let key = "TODO: use some sort of unique key";
        let binding = self.app.insert_binding(key, component, rc_node.clone());

        View {
            app: self.app.clone(),
            key: key.to_string(),
            node: rc_node,
            binding: binding,
            phantom: PhantomData,
        }
    }
}

impl<'doc> Node<'doc> {
    pub fn set(&self, prop: &str, value: &str) {
        self.node.prop_set_str(prop, value);
    }

    pub fn get(&self, prop: &str) -> String {
        self.node.prop_get_str(prop)
    }

    pub fn checked(&self) -> bool {
        self.node.prop_get_i32("checked") != 0
    }
}

impl<'doc, R: 'static + Renderable> View<'doc, R> {
    pub fn set(&self, prop: &str, value: &str) {
        self.node.prop_set_str(prop, value);
    }

    pub fn get(&self, prop: &str) -> String {
        self.node.prop_get_str(prop)
    }

    pub fn checked(&self) -> bool {
        self.node.prop_get_i32("checked") != 0
    }
}

impl<'doc, R: 'static + Renderable> Queryable<'doc> for View<'doc, R> {
    type Q = Self;

    fn query(&self, el: &str) -> Option<Self::Q> {
        self.node.element_query(el).map(|node| {
            View {
                app: self.app.clone(),
                node: Rc::new(node),
                key: self.key.clone(),
                binding: self.binding.clone(),
                phantom: PhantomData,
            }
        })
    }

    fn bind<RR>(&self, el: &str, component: RR) -> View<'doc, RR>
        where RR: 'static + Renderable
    {
        let node = self.node.element_query(el).expect("querySelect returned no result");
        let props = lookup_props(&node, component.props());
        node.html_patch(&component.render(props));

        let rc_node = Rc::new(node);
        let key = "TODO: pick some unique key";
        let binding = self.app.insert_binding(key, component, rc_node.clone());

        View {
            app: self.app.clone(),
            node: rc_node,
            binding: binding,
            key: key.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<'doc, R: 'static + Renderable> View<'doc, R> {
    pub fn on<F>(&self, event: EventType, f: F)
        where F: Fn(Event<Self>) + 'doc
    {
        let app = self.app.clone();
        let key = self.key.clone();
        let binding = self.binding.clone();
        let node = self.node.clone();

        let event_handler = Rc::new(move |evt: webplatform::Event<'doc>, i| {
            let node: View<'doc, R> = View {
                app: app.clone(),
                key: key.clone(),
                node: node.clone(),
                binding: binding.clone(),
                phantom: PhantomData,
            };
            println!("Event fired on {:?} for target {:?}",
                     &node.binding.borrow().node,
                     evt.target);
            let target_node = evt.target.expect("Event did not have a target");
            let event = Event {
                binding: node,
                target: Node {
                    node: target_node,
                    app: app.clone(),
                },
                // FIXME: strange to attach a meaningless index here
                index: i,
            };
            f(event);
            app.process_render_queue();
        });

        // Attach event_handler to the DOM
        let f = event_handler.clone();

        self.node.on(event.name(), move |evt| f(evt, 0));

        // Attach event_handler to binding for future rendering
        self.binding.borrow_mut().add_handler(event.clone(), None, event_handler, vec![self.node.clone()]);
        println!("On handler registered for {:?}", self.node);
    }

    pub fn on_each<F>(&self, event: EventType, el: &str, f: F)
        where F: Fn(Event<Self>) + 'doc
    {
        let app = self.app.clone();
        let key = self.key.clone();
        let binding = self.binding.clone();
        let node = self.node.clone();

        let event_handler = Rc::new(move |evt: webplatform::Event<'doc>, i| {
            let node: View<'doc, R> = View {
                app: app.clone(),
                key: key.clone(),
                node: node.clone(),
                binding: binding.clone(),
                phantom: PhantomData,
            };
            println!("Event fired on {:?} for target {:?}",
                     &node.binding.borrow().node,
                     evt.target);
            let target_node = evt.target.expect("Event did not have a target");
            let event = Event {
                binding: node,
                target: Node {
                    node: target_node,
                    app: app.clone(),
                },
                index: i,
            };
            f(event);
            app.process_render_queue();
        });

        // Attach event_handler to the DOM
        let nodes = self.node.element_query_all(&el);
        for (i, node) in nodes.iter().enumerate() {
            let f = event_handler.clone();
            node.on(event.name(), move |evt| f(evt, i));
        }
        println!("On handlers registered for nodes: {:?}", &nodes);

        // Attach event_handler to binding for future rendering
        self.binding.borrow_mut().add_handler(event.clone(), Some(el.to_owned()), event_handler, nodes.into_iter().map(Rc::new).collect());
    }
}

// **********************************
// More impls
// **********************************

impl<'doc, R: 'static + Renderable> HasBind<'doc> for View<'doc, R> {
    type R = R;

    fn data(&self) -> Ref<R> {
        Ref::map(self.binding.borrow(), |r| r.component())
    }

    fn data_mut(&mut self) -> RefMut<R> {
        // Before handing back mutable the mutable component,
        // enqueue rendering of the original view that owns this data
        let view_id = TypedKey::new::<R>(&self.key);
        self.app.enqueue_render(view_id);
        RefMut::map(self.binding.borrow_mut(), |r| r.component_mut())
    }
}




pub fn init<'a, 'doc: 'a>() -> QuasarApp<'a> {
    QuasarApp {
        document: Rc::new(webplatform::init()),
        app: Rc::new(AppState::new()),
    }
}


pub fn lookup_props<'doc>(node: &HtmlNode<'doc>, keys: &[&'static str]) -> Properties {
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
