use state::{AppState, Binding, DataRef, DataMutRef, TypedKey};
use std::collections::HashMap;
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

pub struct NodeBind<'doc, R> {
    app: Rc<AppState<'doc>>,
    el: String,
    // TODO: probably need the HtmlNode once supporting `query`
    binding: Rc<RefCell<Binding<'doc>>>,
    phantom: PhantomData<R>,
    // TODO: generic marker SingleBind or Multibind to indicate if we can iterate
}

pub struct NodeBindRef<'doc, R, S> {
    app: Rc<AppState<'doc>>,
    el: String,
    // TODO: probably need the HtmlNode once supporting `query`
    binding: Rc<RefCell<Binding<'doc>>>,
    phantom: PhantomData<R>,
    mapper: Rc<Fn(&R) -> &S>,
    // TODO: generic marker SingleBind or Multibind to indicate if we can iterate
}


pub struct SingleBind;
pub struct MultiBind;

// Can we encompass this as a variant of NodeBindRef that can enumerate?
// pub struct NodeBindRefEach<'doc, R, S> {
//     app: Rc<AppState<'doc>>,
//     el: String,
//     binding: Rc<RefCell<Binding<'doc>>>,
//     phantom: PhantomData<R>,
//     mapper: Rc<Fn(&R) -> &S>,
//     // NodeBindRef plus ability to iterate
// }


pub trait Queryable<'doc> {
    type Q: Queryable<'doc>;

    fn query(&self, el: &str) -> Self::Q;
    // fn query_all(&self, el: &str) -> Vec<Self>

    fn bind<R>(&self, el: &str, component: R) -> NodeBind<'doc, R> where R: 'static + Renderable;
    // fn bind_each(&self, el: &str, component: Vec<R>) -> BindEachNode<'doc, R>;
}

pub trait HasBind<'doc> {
    type R: Renderable;

    fn data(&self) -> Ref<Self::R>;
    fn data_mut(&mut self) -> RefMut<Self::R>;

    fn bind_ref<S, F>(&self, el: &str, map_fn: F) -> NodeBindRef<'doc, Self::R, S>
         where S: Renderable + 'static,
               F: 'static + for<'a> Fn(&'a Self::R) -> &'a S;

    fn bind_ref_each<S, F>(&self, el: &str, map_fn: F) -> NodeBindRef<'doc, Self::R, Vec<S>>
        where S: Renderable + 'static,
            F: 'static + for<'a> Fn(&'a Self::R) -> &'a Vec<S>;
}

impl<'doc> Queryable<'doc> for QuasarApp<'doc> {
    type Q = Node<'doc>;
    fn query(&self, el: &str) -> Node<'doc> {
        let node = self.document.element_query(el).expect("querySelect returned no result");

        Node {
            app: self.app.clone(),
            node: node,
        }
    }

    fn bind<R: 'static + Renderable>(&self, el: &str, component: R) -> NodeBind<'doc, R> {
        let node = self.document.element_query(el).expect("querySelector found no results");

        let props = lookup_props(&node, component.props());
        node.html_set(&component.render(props));

        let binding = self.app.insert_binding(el, component, node);

        NodeBind {
            app: self.app.clone(),
            el: "TODO: USE A UUID instead of el".to_string(),
            binding: binding,
            phantom: PhantomData,
        }
    }

}

impl<'doc> Queryable<'doc> for Node<'doc> {
    type Q = Self;

    fn query(&self, el: &str) -> Self::Q {
        let node = self.node.element_query(el).expect("querySelect returned no result");

        Node {
            app: self.app.clone(),
            node: node,
        }
    }

    fn bind<RR>(&self, el: &str, component: RR) -> NodeBind<'doc, RR>
        where RR: 'static + Renderable
    {
        let node = self.node.element_query(el).expect("querySelector found no results");
        let props = lookup_props(&node, component.props());
        node.html_set(&component.render(props));

        let binding = self.app.insert_binding(el, component, node);

        NodeBind {
            app: self.app.clone(),
            el: "TODO: USE A UUID instead of el".to_string(),
            binding: binding,
            phantom: PhantomData,
        }
    }

}


impl<'doc, R: 'static + Renderable> Queryable<'doc> for NodeBind<'doc, R> {
    type Q = Self;

    fn query(&self, el: &str) -> Self::Q {
        let node = self.binding.borrow().node.element_query(el).expect("querySelect returned no result");

        NodeBind {
            app: self.app.clone(),
            el: "TODO: USE A UUID instead of el".to_string(),
            binding: self.binding.clone(),
            phantom: PhantomData,
        }
    }

    fn bind<RR>(&self, el: &str, component: RR) -> NodeBind<'doc, RR>
        where RR: 'static + Renderable
    {
        let node = self.binding.borrow().node.element_query(el).expect("querySelector found no results");
        let props = lookup_props(&node, component.props());
        node.html_set(&component.render(props));

        let binding = self.app.insert_binding(el, component, node);

        NodeBind {
            app: self.app.clone(),
            binding: binding,
            el: el.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<'doc, R: 'static + Renderable> NodeBind<'doc, R> {
    pub fn on<F>(&self, event: EventType, f: F)
        where F: Fn(Event<Self>) + 'doc
    {

            let app = self.app.clone();
            let el = self.el.clone();
            let binding = self.binding.clone();

            let binding_borrow = self.binding.borrow();
            let ref current_node = binding_borrow.node;
            current_node.on(event.name(), move |evt| {
                let node = NodeBind {
                    app: app.clone(),
                    el: el.clone(),
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
                };
                f(event);
                app.process_render_queue();
            });
            println!("On handler registered");
    }
}

// **********************************
// More impls
// **********************************

impl<'doc, R: 'static + Renderable> HasBind<'doc> for NodeBind<'doc, R> {
    type R = R;

    // TODO: this function should quietly create a parent view for updating when the array changes,
    // and instead return Vec<MappedView>
    // Also, will need to provide blanket `impl Renderable for Vec<T> where T:Renderable`
    // pub fn bind_each<RR, VR>(&self, el: &str, components: VR) -> Views<'doc, RR>
    //     where RR: Renderable + 'static,
    //           VR: IntoIterator<Item = RR>,
    // {
    //     let node = self.app.document.element_query(el).expect("querySelector found no results");
    //     let rc_node =  Rc::new(node);

    //     let mut views = Vec::new();
    //     let mut html = String::new();
    //     for component in components {
    //         let props = lookup_props(&rc_node, component.props());
    //         html.push_str(&component.render(props));

    //         let binding = Binding::new(component, node);
    //         let rc_binding = Rc::new(RefCell::new(binding));
    //         {
    //             let view_id = TypedKey::new::<RR>(el);
    //             let mut components = self.app.components.borrow_mut();
    //             components.insert(view_id, rc_component.clone());
    //         }

    //         let view = View {
    //             app: self.app.clone(),
    //             node: rc_node.clone(),
    //             el: el.to_owned(),
    //             component: rc_component.clone(),
    //             phantom: PhantomData,
    //         };
    //         views.push(view);
    //     }
    //     rc_node.html_set(&html);
    //     Views {
    //         views: Rc::new(views),
    //         // handlers: Rc::new(RefCell::new(Vec::new())),
    //     }
    // }


    fn bind_ref<S, F>(&self, el: &str, map_fn: F) -> NodeBindRef<'doc, R, S>
        where S: Renderable + 'static,
              F: Fn(&R) -> &S + 'static,
    {
        let node = self.binding.borrow().node.element_query(el).expect("querySelector found no results");
        let parent_component = self.data();
        let component = map_fn(&parent_component);
        let props = lookup_props(&node, component.props());
        node.html_set(&component.render(props));

        {
            let mut binding = self.binding.borrow_mut();
            binding.add(node, &map_fn);
        }

        NodeBindRef {
            app: self.app.clone(),
            binding: self.binding.clone(),
            el: el.to_owned(),
            phantom: PhantomData,
            mapper: Rc::new(map_fn),
        }
    }


    fn bind_ref_each<S, F>(&self, el: &str, map_fn: F) -> NodeBindRef<'doc, R, Vec<S>>
        where S: Renderable + 'static,
              F: 'static + Fn(&R) -> &Vec<S>,
    {
        let node = self.binding.borrow().node.element_query(el).expect("querySelector found no results");
        let parent_component = self.data();

        let components = map_fn(&parent_component);

        // TODO: rethink this as part of props cleanup
        let props = lookup_props(&node, components.props());

        let mut html = Vec::with_capacity(components.len());
        for c in components {
            html.push(c.render(props.clone()));

        }
        node.html_set(&html.concat());

        {
            let mut binding = self.binding.borrow_mut();
            binding.add(node, &map_fn);
        }

        // TODO: children should get a mapper for each component:: Rc::new(move |ref data| { &mapper(&data)[i] }),
        NodeBindRef {
            app: self.app.clone(),
            binding: self.binding.clone(),
            el: el.to_owned(),
            phantom: PhantomData,
            mapper: Rc::new(map_fn),
        }
    }




    fn data(&self) -> Ref<R> {
        Ref::map(self.binding.borrow(), |r| r.component())
    }

    fn data_mut(&mut self) -> RefMut<R> {
        // Before handing back mutable the mutable component,
        // enqueue rendering of the original view that owns this data
        let view_id = TypedKey::new::<R>(&self.el);
        self.app.enqueue_render(view_id);
        RefMut::map(self.binding.borrow_mut(), |r| r.component_mut())
    }
}




pub fn init<'a, 'doc: 'a>() -> QuasarApp<'a> {
    QuasarApp {
        document: Rc::new(webplatform::init()),
        app: Rc::new(AppState::new())
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