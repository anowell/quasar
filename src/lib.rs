extern crate webplatform;
extern crate rustc_serialize;
extern crate uuid;

#[macro_use]
extern crate downcast_rs;

#[cfg(feature = "mustache")]
extern crate mustache;

mod events;
mod components;
mod state;
mod node;
mod view;
mod app;

pub use events::EventType;
pub use components::{Component, Properties, Renderable};
pub use app::{init, QuasarApp, AppContext};
pub use node::Node;
pub use view::View;
pub use rustc_serialize::json::Json;
use webplatform::HtmlNode;

use std::cell::{Ref, RefMut};

#[cfg(feature = "mustache")]
pub use mustache::compile_str;


impl<'doc> QuasarApp<'doc> {
    // pub fn bind_all<R: 'static + Renderable>(&self, el: &str, component: R) -> Views<'doc, R> {
    //     let nodes = self.document.element_query_all(el);
    //     if nodes.is_empty() {
    //         panic!("querySelectorAll found no results for {}", &el);
    //     }

    //     let rc_component = Rc::new(RefCell::new(component));
    //     {
    //         let view_id = TypedKey::new::<R>(el);
    //         let mut components = self.components.borrow_mut();
    //         components.insert(view_id, rc_component.clone());
    //     }

    //     let mut views = Vec::new();
    //     for node in nodes {
    //         {
    //             let component = rc_component.borrow();
    //             let props = lookup_props(&node, component.props());
    //             node.html_set(&component.render(props));
    //         }
    //         let view = View {
    //             app: self.clone(),
    //             node: Rc::new(node),
    //             el: el.to_owned(),
    //             component: rc_component.clone(),
    //             phantom: PhantomData,
    //         };
    //         views.push(view);
    //     }
    //     Views {
    //         views: Rc::new(views),
    //         // handlers: Rc::new(RefCell::new(Vec::new())),
    //     }
    // }

    pub fn spin(self) {
        webplatform::spin();
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



pub struct Event<'doc, N> {
    /// The node that triggered the event
    pub target: Node<'doc>,
    // The node the event was attached to (may include data binding)
    pub binding: N,
    // The globally shared app context (provides access to document root)
    pub app: AppContext<'doc>,
    // The target's index offset when event was attached multiple times for a selector
    pub index: usize,
}