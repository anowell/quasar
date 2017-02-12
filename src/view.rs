use state::{AppState, Binding, TypedKey};
use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
use std::marker::PhantomData;
use webplatform::{self, HtmlNode};
use uuid::Uuid;

use {Queryable, HasBind, Renderable, Event, EventType, AppContext, Node};


pub struct View<'doc, R> {
    app: Rc<AppState<'doc>>,
    node: Rc<HtmlNode<'doc>>,
    key: String,
    binding: Rc<RefCell<Binding<'doc>>>,
    phantom: PhantomData<R>, // TODO: generic marker SingleBind or Multibind to indicate if we can iterate
}


impl<'doc, R: 'static + Renderable> View<'doc, R> {
    #![doc(hidden)]
    pub fn new(app: Rc<AppState<'doc>>, node: Rc<HtmlNode<'doc>>, key: String, binding: Rc<RefCell<Binding<'doc>>>) -> View<'doc, R>
        // where RR: 'static + Renderable
    {
        View {
            app: app,
            node: node,
            key: key,
            binding: binding,
            phantom: PhantomData,
        }
    }

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
                app: AppContext::new(app.clone(), Some(TypedKey::new::<R>(&key))),
                binding: node,
                target: Node::new(app.clone(), Rc::new(target_node)),
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
                app: AppContext::new(app.clone(), Some(TypedKey::new::<R>(&key))),
                binding: node,
                target: Node::new(app.clone(), Rc::new(target_node)),
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
        let rc_node = Rc::new(node);
        let render_node = Node::new(self.app.clone(), rc_node.clone());
        let key = Uuid::new_v4().to_string();

        let app_context = AppContext::new(self.app.clone(), Some(TypedKey::new::<RR>(&key)));
        rc_node.html_patch(&component.render(&render_node, &app_context));

        let binding = self.app.insert_binding(&key, component, rc_node.clone());

        View {
            app: self.app.clone(),
            node: rc_node,
            binding: binding,
            key: key.to_owned(),
            phantom: PhantomData,
        }
    }
}

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