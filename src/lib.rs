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

pub use events::{EventType};
pub use components::{Component, Properties, Renderable};
pub use nodes::{init, QuasarApp, Node, View, Queryable, HasBind};
pub use rustc_serialize::json::Json;

use nodes::lookup_props;
use state::{AppState, DataRef, DataMutRef, TypedKey};
use std::rc::Rc;
use webplatform::HtmlNode;

#[cfg(feature = "mustache")]
pub use mustache::compile_str;








impl<'doc> QuasarApp<'doc> {
    // pub fn bind<R: 'static + Renderable>(&self, el: &str, component: R) -> View<'doc, R> {
    //     let node = self.document.element_query(el).expect("querySelector found no results");

    //     let props = lookup_props(&node, component.props());
    //     node.html_set(&component.render(props));

    //     let binding = self.app.insert_binding(el, component, node);

    //     View::new(self.app.clone(), binding)
    // }
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
    node: Rc<HtmlNode<'doc>>,
}

impl<'doc> AppContext<'doc> {
    /// Get app data for a specific key
    ///
    /// This will flag the view in scope as an observer of this data bucket,
    ///   and any modifications to data at this key will cause this view to be re-rendered.
    pub fn data<T: 'static>(&self, key: &str) -> DataRef<T> {
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
    pub fn data_mut<T: 'static>(&mut self, key: &str) -> DataMutRef<T> {
        let type_id = TypedKey::new::<T>(&key);
        self.app.add_observer(type_id, self.view_id.clone());
        self.app.data_mut(key)
    }
}

/// A collection of `View`s returned from a query selector
//pub struct Views<'doc, R> {
    // views: Rc<Vec<View<'doc, R>>>,
    // Views may have multiple handlers, hence Vec
    // We want interior mutability, hence RefCell
    // A handler may map to multiple views
    // handlers: Rc<RefCell<Vec<Box<Fn(Event<R>) + 'doc>>>>,
// }

// impl <'a, 'doc, R> IntoIterator for &'a Views<'doc, R> {
//     type Item = &'a View<'doc, R>;
//     type IntoIter = ::std::slice::Iter<'a, View<'doc, R>>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.views.iter()
//     }
// }

// impl <'doc, R> FromIterator<View<'doc, R>> for Views<'doc, R> {
//     fn from_iter<I: IntoIterator<Item=View<'doc, R>>>(iter: I) -> Self {
//         let mut views = Vec::new();
//         for view in iter {
//             views.push(view);
//         }

//         Views {
//             views: Rc::new(views),
//             // handlers: Rc::new(RefCell::new(Vec::new()))
//         }
//     }
// }

// impl<'doc, R: Renderable + 'static> Views<'doc, R> {
    // pub fn on<F>(&self, event: EventType, f: F)
    //     where F: Fn(Event<R>) + 'doc
    // {
    //     // Insert the handler into self and return it's index
    //     let offset = {
    //         let mut handlers = self.handlers.borrow_mut();
    //         handlers.push(Box::new(f));
    //         handlers.len() - 1
    //     };


    //     // For each view, setup a unique 'on' handler
    //     for v in self.views.iter() {
    //         println!("attaching handler to view: {:?}", &v.node);
    //         let handlers = self.handlers.clone();
    //         let app = v.app.clone();
    //         let el = v.el.clone();
    //         let node = v.node.clone();
    //         let component = v.component.clone();
    //         let views = self.views.clone();

    //         v.node.on(event.name(), move |evt| {
    //             let handlers = handlers.clone();

    //             let view = View {
    //                 app: app.clone(),
    //                 el: el.clone(),
    //                 node: node.clone(),
    //                 component: component.clone(),
    //                 phantom: PhantomData,
    //             };

    //             // Process the event with the target and originating view
    //             println!("Event fired on {:?} for target {:?}",
    //                      &view.node,
    //                      evt.target);
    //             {
    //                 let event = Event {
    //                     app: AppContext {
    //                         app: app.clone(),
    //                         view_id: TypedKey::new::<R>(&el),
    //                         node: node.clone(),
    //                     },
    //                     target: Element { node: evt.target.expect("Event did not have a target"), parent_view: Rc::new(view) },
    //                     view: view,
    //                 };
    //                 let inner_handlers = handlers.borrow();
    //                 inner_handlers[offset](event);
    //             }

    //             // Re-render all the views for this component instance
    //             // TODO: remove this section
    //             // instead, bind should add all these views as observers of this component data
    //             // and let `data_mut` schedule re-render which could be handled here or in tick loop
    //             for v in views.iter() {
    //                 let node = v.node.clone();
    //                 let component = v.component.clone();
    //                 let rendered = {
    //                     let component = component.borrow();
    //                     let props = lookup_props(&node, component.props());
    //                     component.render(props)
    //                 };
    //                 node.html_set(&rendered);
    //             }
    //             app.process_render_queue()
    //         });
    //     }
    //     println!("{} On handlers registered", self.views.len());
    // }
// }





// impl<'doc, R: 'static + Renderable, S: 'static + Renderable> MappedView<'doc, R, S> {


//     pub fn query(&self, el: &str) -> Element<'doc, MappedView<'doc, R>> {
//         let node = self.node.element_query(el).expect("querySelect returned no result");

//         Element {
//             node: Rc::new(node),
//             parent_view: self.clone(),
//         }
//     }

//     pub fn on<F>(&self, event: EventType, f: F)
//         where F: Fn(Event<R>) + 'doc
//     {}

// }





pub struct Event<'doc, N> {
    /// The node that triggered the event
    pub target: Node<'doc>,
    // The node the event was attached to (may include data binding)
    pub binding: N,
    // The target's index offset when event was attached multiple times for a selector
    pub index: usize,
}

// impl <'doc, R: Renderable + 'doc> Event<'doc, R> {
//     pub fn app(&self) -> AppContext<'doc> {
//         AppContext {
//             app: self.view.app.clone(),
//             view_id: TypedKey::new::<R>(&self.view.el),
//             node: self.view.node.clone(),
//         }
//     }

//     pub fn view(&self) -> View<'doc, R> {
//         self.view.clone()
//     }

//     pub fn target(&self) -> Element<'doc, R> {
//         Element {
//             node: self.target.clone(),
//             parent_view: self.view.clone(),
//         }
//     }
// }

// pub struct Elements<'doc, R> {
//     nodes: Vec<HtmlNode<'doc>>,
//     parent_view: Rc<View<'doc, R>>,
// }



// impl<'doc, V, R> Queryable for Element<'doc, V, R> {
//     type Q = Element<'doc, V, R>;

//     fn query(&self, el: &str) -> Self::Q {
//         let binding = self.binding.borrow();
//         let node = self.node.element_query(el).expect("querySelect returned no result");

//         Element {
//             node: Rc::new(node),
//             parent_view: self.parent_viewclone(),
//             phantom: PhantomData,
//         }
//     }
// }

// impl<'doc, R, V> Element<'doc, V, R> {
//     pub fn set(&self, prop: &str, value: &str) {
//         self.node.prop_set_str(prop, value);
//     }

//     pub fn get(&self, prop: &str) -> String {
//         self.node.prop_get_str(prop)
//     }
// }

// impl<'doc, R: 'static + Renderable> View<'dNodBindRef, RefViewEach<'doc, R>, R> {
//     pub fn on<F>(&self, event: EventType, f: F)
//         where F: Fn(Event<View<'doc, R>, R>) + 'doc
//     {
//         {
//             let node = self.node.clone();
//             let parent_view = self.parent_view.clone();

//             // we attach the event to self.node, not self.parent_view.node
//             self.node.on(event.name(), move |evt| {
//                 println!("Event fired on {:?} for target {:?}",
//                          &node,
//                          evt.target);
//                 let target_node = evt.target.expect("Event did not have a target");
//                 let event = Event {
//                     view: parent_view.clone(),
//                     target: Element {
//                         node: Rc::new(target_node),
//                         parent_view: parent_view.clone(),
//                         phantom: PhantomData,
//                     },
//                 };
//                 f(event);
//                 parent_view.app.process_render_queue();
//             });
//             println!("On handler registered");
//         }
//     }
// }

// impl<'doc, R: 'static + Renderable, S: 'static + Renderable> Element<'doc, MappedView<'doc, R, S>, R> {
//     pub fn on<F>(&self, event: EventType, f: F)
//         where F: Fn(Event<MappedView<'doc, R, S>, R>) + 'doc
//     {
//         {
//             let node = self.node.clone();
//             let parent_view = self.parent_view.clone();

//             // we attach the event to self.node, not self.parent_view.node
//             self.node.on(event.name(), move |evt| {
//                 println!("Event fired on {:?} for target {:?}",
//                          &node,
//                          evt.target);
//                 let target_node = evt.target.expect("Event did not have a target");
//                 let event = Event {
//                     view: parent_view.clone(),
//                     target: Element {
//                         node: Rc::new(target_node),
//                         parent_view: parent_view.clone(),
//                         phantom: PhantomData,
//                     },
//                 };
//                 f(event);
//                 parent_view.view.app.process_render_queue();
//             });
//             println!("On handler registered");
//         }
//     }
// }
