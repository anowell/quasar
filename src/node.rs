use state::{AppState, TypedKey};
use std::rc::Rc;
use webplatform::{self, HtmlNode};
use uuid::Uuid;

use {Queryable, Renderable, Properties, Event, EventType, View, AppContext, lookup_props};

pub struct Node<'doc> {
    app: Rc<AppState<'doc>>,
    node: Rc<HtmlNode<'doc>>,
}


impl<'doc> Queryable<'doc> for Node<'doc> {
    type Q = Self;

    fn query(&self, el: &str) -> Option<Self::Q> {
        self.node.element_query(el).map(|node| {
            Node {
                app: self.app.clone(),
                node: Rc::new(node),
            }
        })
    }

    fn bind<RR>(&self, el: &str, component: RR) -> View<'doc, RR>
        where RR: 'static + Renderable
    {
        let node = self.node.element_query(el).expect("querySelector found no results");
        let rc_node = Rc::new(node);

        let render_node = Node { app: self.app.clone(), node: rc_node.clone() };
        let key = Uuid::new_v4().to_string();

        let app_context = AppContext::new(self.app.clone(), Some(TypedKey::new::<RR>(&key)));
        rc_node.html_patch(&component.render(&render_node, &app_context));

        let binding = self.app.insert_binding(&key, component, rc_node.clone());

        View::new(self.app.clone(), rc_node, key, binding)
    }
}

impl<'doc> Node<'doc> {
    #![doc(hidden)]
    pub fn new(app: Rc<AppState<'doc>>, node: Rc<HtmlNode<'doc>> ) -> Node<'doc> {
        Node {
            app: app,
            node: node,
        }
    }

    pub fn query_all(&self, el: &str) -> Vec<Node<'doc>> {
        self.node.element_query_all(el).into_iter().map(|node| {
            Node {
                app: self.app.clone(),
                node: Rc::new(node),
            }
        }).collect()
    }

    pub fn on<F>(&self, event: EventType, f: F)
        where F: Fn(Event<Self>) + 'doc
    {
        let app = self.app.clone();
        let node = self.node.clone();

        let event_handler = Rc::new(move |evt: webplatform::Event<'doc>, i| {
            let node = Node {
                app: app.clone(),
                node: node.clone(),
            };
            println!("Event fired on {:?} for target {:?}",
                     &node.node,
                     evt.target);
            let target_node = evt.target.expect("Event did not have a target");
            let event = Event {
                app: AppContext::new(app.clone(), None),
                binding: node,
                target: Node {
                    node: Rc::new(target_node),
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
        println!("On handler registered for {:?}", self.node);
    }

    pub fn set(&self, prop: &str, value: &str) {
        self.node.prop_set_str(prop, value);
    }

    pub fn get(&self, prop: &str) -> String {
        self.node.prop_get_str(prop)
    }

    pub fn get_attr(&self, attr: &str) -> String {
        self.node.attr_get_str(attr)
    }

    pub fn get_properties(&self, keys: &[&'static str]) -> Properties {
        lookup_props(&self.node, keys)
    }

    pub fn checked(&self) -> bool {
        self.node.prop_get_i32("checked") != 0
    }

    pub fn value(&self) -> String {
        self.get("value")
    }

}