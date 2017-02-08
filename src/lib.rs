extern crate webplatform;
extern crate rustc_serialize;

#[macro_use]
extern crate downcast_rs;

#[cfg(feature = "mustache")]
extern crate mustache;

mod events;
mod components;
mod state;
mod nodes;

pub use events::EventType;
pub use components::{Component, Properties, Renderable};
pub use nodes::{init, QuasarApp, Node, View, Queryable, HasBind};
pub use rustc_serialize::json::Json;

use state::{AppState, DataRef, DataMutRef, TypedKey};
use std::rc::Rc;

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


/// Provides select access to the global `QuasarApp` object in the context of a specific `View`
pub struct AppContext<'doc> {
    app: Rc<AppState<'doc>>,
    view_id: TypedKey,
}

impl<'doc> AppContext<'doc> {
    #![doc(hidden)]
    pub fn new(app: Rc<AppState<'doc>>, view_id: TypedKey) -> AppContext<'doc> {
        AppContext {
            app: app,
            view_id: view_id,
        }
    }

    /// Get app data for a specific key
    ///
    /// This will flag the view in scope as an observer of this data bucket,
    ///   and any modifications to data at this key will cause this view to be re-rendered.
    pub fn data<T: 'static>(&self, key: &str) -> Option<DataRef<T>> {
        let type_id = TypedKey::new::<T>(&key);
        self.app.add_observer(type_id, self.view_id.clone());
        self.app.data(key)
    }

    /// Get app data for a specific key
    ///
    /// This will flag the view in scope as an observer of this data bucket,
    ///   and any modifications to data at this key will cause this view to be re-rendered.
    /// It will also cause all observers of this view to be re-rendered after processing
    ///   of the current event is finished.
    pub fn data_mut<T: 'static>(&mut self, key: &str) -> Option<DataMutRef<T>> {
        let type_id = TypedKey::new::<T>(&key);
        self.app.add_observer(type_id, self.view_id.clone());
        self.app.data_mut(key)
    }
}

pub struct Event<'doc, N> {
    /// The node that triggered the event
    pub target: Node<'doc>,
    // The node the event was attached to (may include data binding)
    pub binding: N,
    // The target's index offset when event was attached multiple times for a selector
    pub index: usize,
}