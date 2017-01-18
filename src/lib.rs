extern crate webplatform;
extern crate rustc_serialize;

#[macro_use]
extern crate downcast_rs;

#[cfg(feature = "mustache")]
extern crate mustache;

mod events;
mod components;
mod state;

pub use events::{EventType};
pub use components::{Component, Properties, Renderable};
pub use rustc_serialize::json::Json;

use state::{AppState, Binding, DataRef, DataMutRef, TypedKey};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
use std::marker::PhantomData;
use webplatform::{Document, HtmlNode};

#[cfg(feature = "mustache")]
pub use mustache::compile_str;

pub fn init<'a, 'doc: 'a>() -> QuasarApp<'a> {
    QuasarApp {
        document: Rc::new(webplatform::init()),
        app: Rc::new(AppState::new())
    }
}




/// The main app object instantiated by calling `quasar::init()`
pub struct QuasarApp<'doc> {
    document: Rc<Document<'doc>>,
    app: Rc<AppState<'doc>>,
}


impl<'doc> QuasarApp<'doc> {
    pub fn bind<R: 'static + Renderable>(&self, el: &str, component: R) -> View<'doc, R> {
        let node = self.document.element_query(el).expect("querySelector found no results");

        let props = lookup_props(&node, component.props());
        node.html_set(&component.render(props));

        let binding = self.app.insert_binding(el, component, node);

        View {
            app: self.app.clone(),
            el: el.to_owned(),
            binding: binding,
            phantom: PhantomData,
        }
    }
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
pub struct Views<'doc, R> {
    views: Rc<Vec<View<'doc, R>>>,
    // Views may have multiple handlers, hence Vec
    // We want interior mutability, hence RefCell
    // A handler may map to multiple views
    // handlers: Rc<RefCell<Vec<Box<Fn(Event<R>) + 'doc>>>>,
}

impl <'a, 'doc, R> IntoIterator for &'a Views<'doc, R> {
    type Item = &'a View<'doc, R>;
    type IntoIter = ::std::slice::Iter<'a, View<'doc, R>>;

    fn into_iter(self) -> Self::IntoIter {
        self.views.iter()
    }
}

impl <'doc, R> FromIterator<View<'doc, R>> for Views<'doc, R> {
    fn from_iter<I: IntoIterator<Item=View<'doc, R>>>(iter: I) -> Self {
        let mut views = Vec::new();
        for view in iter {
            views.push(view);
        }

        Views {
            views: Rc::new(views),
            // handlers: Rc::new(RefCell::new(Vec::new()))
        }
    }
}

impl<'doc, R: Renderable + 'static> Views<'doc, R> {
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
    app: Rc<AppState<'doc>>,
    el: String,
    binding: Rc<RefCell<Binding<'doc>>>,
    phantom: PhantomData<R>,
}

pub struct MappedView<'doc, R, S> {
    view: View<'doc, R>,
    mapper: Rc<Fn(&R) -> &S>,
}

impl<'doc, R: 'static + Renderable, S: 'static + Renderable> MappedView<'doc, R, S> {
    // not public - for internal convenience only
    fn clone(&self) -> MappedView<'doc, R, S> {
        MappedView {
            view: self.view.clone(),
            mapper: self.mapper.clone(),
        }
    }


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

}

impl<'doc, R: 'static + Renderable> View<'doc, R> {
    pub fn query(&self, el: &str) -> Element<'doc, Self, R> {
        let binding = self.binding.borrow();
        let node = binding.node.element_query(el).expect("querySelect returned no result");

        Element {
            node: Rc::new(node),
            parent_view: self.clone(),
            phantom: PhantomData,
        }
    }

    // not public - for internal convenience only
    fn clone(&self) -> View<'doc, R> {
        View {
            app: self.app.clone(),
            el: self.el.clone(),
            binding: self.binding.clone(),
            phantom: PhantomData,
        }
    }

    pub fn on<F>(&self, event: EventType, f: F)
        where F: Fn(Event<Self, R>) + 'doc
    {
            let view = self.clone();
            let binding_borrow = self.binding.borrow();
            let ref current_node = binding_borrow.node;
            current_node.on(event.name(), move |evt| {
                println!("Event fired on {:?} for target {:?}",
                         &view.binding.borrow().node,
                         evt.target);
                let target_node = evt.target.expect("Event did not have a target");
                let event = Event {
                    view: view.clone(),
                    target: Element {
                        node: Rc::new(target_node),
                        parent_view: view.clone(),
                        phantom: PhantomData,
                    },
                };
                f(event);
                view.app.process_render_queue();
            });
            println!("On handler registered");
    }

    pub fn bind<RR>(&self, el: &str, component: RR) -> View<'doc, RR>
        where RR: 'static + Renderable
    {

        let node = self.binding.borrow().node.element_query(el).expect("querySelector found no results");
        let props = lookup_props(&node, component.props());
        node.html_set(&component.render(props));

        let binding = self.app.insert_binding(el, component, node);

        View {
            app: self.app.clone(),
            binding: binding,
            el: el.to_owned(),
            phantom: PhantomData,
        }
    }

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


    pub fn bind_map<S, F>(&self, el: &str, map_fn: F) -> MappedView<'doc, R, S>
        where S: Renderable + 'static,
              F: 'static + Fn(&R) -> &S,
    {
        let node = self.binding.borrow().node.element_query(el).expect("querySelector found no results");
        let parent_component = self.data();
        let component = map_fn(&parent_component);
        let props = lookup_props(&node, component.props());
        node.html_set(&component.render(props));

        {
            let mut binding = self.binding.borrow_mut();
            binding.add(node, &map_fn);
            // TODO: surface index to the MappedView, maybe?
        }

        MappedView {
            view: View {
                app: self.app.clone(),
                binding: self.binding.clone(),
                el: el.to_owned(),
                phantom: PhantomData,
            },
            mapper: Rc::new(map_fn),
        }
    }


    pub fn bind_map_each<S, F>(&self, el: &str, map_fn: F) -> Vec<MappedView<'doc, R, S>>
        where S: Renderable + 'static,
              F: 'static + Fn(&R) -> &Vec<S>,
    {
        let node = self.binding.borrow().node.element_query(el).expect("querySelector found no results");
        let mut views = Vec::new();
        let rc_map_fn = Rc::new(map_fn);

        {
            let parent_component = self.data();
            let components = rc_map_fn(&parent_component);
            let mut html = String::new();
            for (i, component) in components.iter().enumerate() {
                let props = lookup_props(&node, component.props());
                html.push_str(&component.render(props));

                let mapper = rc_map_fn.clone();
                let view = MappedView {
                    view: View {
                        app: self.app.clone(),
                        binding: self.binding.clone(),
                        el: el.to_owned(),
                        phantom: PhantomData,
                    },
                    // Convert the `Fn(&R) -> &[S]` into a `Fn(&R) -> S` for each S
                    mapper: Rc::new(move |ref data| { &mapper(&data)[i] }),
                };
                views.push(view);
            }
            node.html_set(&html);
        }


        {
            let mut binding = self.binding.borrow_mut();
            binding.add(node, &*rc_map_fn);
        }

        views
    }


    pub fn data(&self) -> Ref<R> {
        Ref::map(self.binding.borrow(), |r| r.component())
    }

    pub fn data_mut(&mut self) -> RefMut<R> {
        // Before handing back mutable the mutable component,
        // enqueue rendering of the original view that owns this data
        let view_id = TypedKey::new::<R>(&self.el);
        self.app.enqueue_render(view_id);
        RefMut::map(self.binding.borrow_mut(), |r| r.component_mut())
    }
}


pub struct Event<'doc, V, R> {
    pub target: Element<'doc, V, R>,
    pub view: V,
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


pub struct Element<'doc, V, R> {
    node: Rc<HtmlNode<'doc>>,
    parent_view: V,
    phantom: PhantomData<R>,
}

impl<'doc, R, V> Element<'doc, V, R> {
    pub fn set(&self, prop: &str, value: &str) {
        self.node.prop_set_str(prop, value);
    }

    pub fn get(&self, prop: &str) -> String {
        self.node.prop_get_str(prop)
    }
}

impl<'doc, R: 'static + Renderable> Element<'doc, View<'doc, R>, R> {
    pub fn on<F>(&self, event: EventType, f: F)
        where F: Fn(Event<View<'doc, R>, R>) + 'doc
    {
        {
            let node = self.node.clone();
            let parent_view = self.parent_view.clone();

            // we attach the event to self.node, not self.parent_view.node
            self.node.on(event.name(), move |evt| {
                println!("Event fired on {:?} for target {:?}",
                         &node,
                         evt.target);
                let target_node = evt.target.expect("Event did not have a target");
                let event = Event {
                    view: parent_view.clone(),
                    target: Element {
                        node: Rc::new(target_node),
                        parent_view: parent_view.clone(),
                        phantom: PhantomData,
                    },
                };
                f(event);
                parent_view.app.process_render_queue();
            });
            println!("On handler registered");
        }
    }
}

impl<'doc, R: 'static + Renderable, S: 'static + Renderable> Element<'doc, MappedView<'doc, R, S>, R> {
    pub fn on<F>(&self, event: EventType, f: F)
        where F: Fn(Event<MappedView<'doc, R, S>, R>) + 'doc
    {
        {
            let node = self.node.clone();
            let parent_view = self.parent_view.clone();

            // we attach the event to self.node, not self.parent_view.node
            self.node.on(event.name(), move |evt| {
                println!("Event fired on {:?} for target {:?}",
                         &node,
                         evt.target);
                let target_node = evt.target.expect("Event did not have a target");
                let event = Event {
                    view: parent_view.clone(),
                    target: Element {
                        node: Rc::new(target_node),
                        parent_view: parent_view.clone(),
                        phantom: PhantomData,
                    },
                };
                f(event);
                parent_view.view.app.process_render_queue();
            });
            println!("On handler registered");
        }
    }
}
