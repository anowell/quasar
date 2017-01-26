use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
use std::ops::{Deref, DerefMut};
use webplatform::{self, HtmlNode};

use {EventType, Renderable, lookup_props};

pub struct Handler<'doc> {
    el: Option<String>,
    event_type: EventType,
    event_handler: Rc<Fn(webplatform::Event<'doc>, usize) + 'doc>,
}

pub struct Binding<'doc> {
    pub node: Rc<HtmlNode<'doc>>,
    component: Box<Renderable>,
    // FIXME: Store selector and handlers, and blindly reapply handlers after rerender until we can patch DOM more conservatively
    handlers: Vec<Handler<'doc>>,

    // FIXME: These shared_binds are fundamentally broken. Node rerender will crussh nodes.
    // option 1: virutal dom, where the shared_bind nodes are rendered to virt dom before parent is rendered to DOM
    // option 2: store el, and just rerender after parent blows away children - still breaks event handlers
    //shared_binds: Vec<(HtmlNode<'doc>, *const Renderable)>,
}

impl<'doc> Binding<'doc> {
    pub fn new<R: 'static + Renderable>(component: R, node: Rc<HtmlNode<'doc>>) -> Binding<'doc> {
        Binding {
            component: Box::new(component),
            node: node,
            handlers: vec![],
        }
    }

    // pub fn add_bind<R, S, F>(&mut self, node: HtmlNode<'doc>, map_fn: &F)
    //     where F: 'static + Fn(&R) -> &S,
    //           R: Renderable + 'static,
    //           S: Renderable + 'static,
    // {
    //     let component: &R = self.component.downcast_ref().unwrap();
    //     let ptr: *const Renderable = map_fn(&component);
    //     self.shared_binds.push((node, ptr));
    // }

    pub fn add_handler(&mut self, event_type: EventType, el: Option<String>, event_handler: Rc<Fn(webplatform::Event<'doc>, usize) + 'doc>) {
        let handler = Handler {
            el: el,
            event_type: event_type,
            event_handler: event_handler,
        };
        self.handlers.push(handler);
    }


    pub fn component<R>(&self) -> &R where R: Renderable {
        self.component.downcast_ref().unwrap()
    }

    pub fn component_mut<R>(&mut self) -> &mut R where R: Renderable {
        self.component.downcast_mut().unwrap()
    }
}

// Map data_id to data
type DataStore = HashMap<TypedKey, Box<Any>>;

// Map view_id to binding
type BindingStore<'doc> = HashMap<TypedKey, Rc<RefCell<Binding<'doc>>>>;

// Map data_id to view_ids that are observing said data
type ObserverStore = HashMap<TypedKey, Vec<TypedKey>>;

// Set of view_id that need rerendered
type RenderQueue = Vec<TypedKey>;



pub struct AppState<'doc> {
    bindings: Rc<RefCell<BindingStore<'doc>>>,
    state: Rc<RefCell<DataStore>>,
    observers: Rc<RefCell<ObserverStore>>,
    render_queue: Rc<RefCell<RenderQueue>>,
}

impl<'doc> AppState<'doc> {
    pub fn new() -> AppState<'doc> {
        AppState {
            bindings: Rc::new(RefCell::new(HashMap::new())),
            state: Rc::new(RefCell::new(HashMap::new())),
            observers: Rc::new(RefCell::new(HashMap::new())),
            render_queue: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn data<T: 'static>(&self, key: &str) -> DataRef<T> {
        let data_id = TypedKey::new::<T>(key);
        let owned_ref = Ref::map(self.state.borrow(), |state| {
            let entry = state.get(&data_id).expect("Failed to get state");
            entry.downcast_ref().unwrap()
        });
        DataRef {
            reference: &*owned_ref,
            _owner: owned_ref,
        }
    }

    pub fn data_mut<T: 'static>(&self, key: &str) -> DataMutRef<T> {
        // Look up observers, and enqueue them for re-render
        let data_id = TypedKey::new::<T>(key);
        {
            let observers = self.observers.borrow();
            if let Some(partition_observers) = observers.get(&data_id) {
                let mut queue = self.render_queue.borrow_mut();
                for observer in partition_observers {
                    queue.push(observer.clone());
                }
            }
        }


        let mut owned_ref = RefMut::map(self.state.borrow_mut(), |mut state| {
            let mut entry = state.get_mut(&data_id).expect("Failed to get mutable state");
            entry.downcast_mut::<T>().unwrap()
        });
        DataMutRef {
            reference: &mut *owned_ref,
            _owner: owned_ref,
        }
    }

    pub fn insert_binding<R: 'static + Renderable>(&self, key: &str, component: R, node: Rc<HtmlNode<'doc>>) -> Rc<RefCell<Binding<'doc>>> {
        let binding = Binding::new(component, node);
        let rc_binding = Rc::new(RefCell::new(binding));
        {
            let view_id = TypedKey::new::<R>(key);
            let mut bindings = self.bindings.borrow_mut();
            bindings.insert(view_id, rc_binding.clone());
        }
        rc_binding
    }

    pub fn enqueue_render(&self, view_id: TypedKey) {
        let mut queue = self.render_queue.borrow_mut();
        queue.push(view_id);
    }

    pub fn add_observer(&self, data_id: TypedKey, view_id: TypedKey) {
        let mut observers = self.observers.borrow_mut();
        let mut partition = observers.entry(data_id).or_insert_with(|| Vec::new());
        partition.push(view_id);
    }

    pub fn process_render_queue(&self) {
        let mut queue = self.render_queue.borrow_mut();
        println!("Processing render queue (len={})", queue.len());
        let bindings = self.bindings.borrow();
        for view_id in queue.iter() {
            let binding = bindings.get(&view_id).expect("failed to get binding for view");
            let binding = binding.borrow();
            let ref component = binding.component;

            // Rerender the main binding
            println!("Rerender node {:?}", &binding.node);
            let props = lookup_props(&binding.node, component.props());
            binding.node.html_set(&component.render(props));

            // Attach any event handlers.
            // FIXME: This isn't very well done, since we should only need to attach
            // them for nodes that were added. But since we blew away the entire DOM,
            // we'll just reattach them all.
            for handler in &binding.handlers {
                if let Some(ref el) = handler.el {
                    let nodes = binding.node.element_query_all(&el);
                    for (i, node) in nodes.iter().enumerate() {
                        let f = handler.event_handler.clone();
                        node.on(handler.event_type.name(), move |event| { f(event, i) });
                    }
                    println!("On handlers REregistered for nodes: {:?}", &nodes);
                }
            }
        }
        queue.clear();
    }

}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct TypedKey {
    tid: TypeId,
    key: String,
}

impl TypedKey {
    pub fn new<R: 'static>(key: &str) -> TypedKey {
        TypedKey {
            tid: TypeId::of::<R>(),
            key: key.to_owned(),
        }
    }
}

/// Reference to generic app data
pub struct DataRef<'a, T: 'a> {
    _owner: Ref<'a, T>,
    reference: *const T,
}

/// Mutable reference to generic app data
pub struct DataMutRef<'a, T: 'a> {
    _owner: RefMut<'a, T>,
    reference: *mut T,
}

impl<'a, T> Deref for DataRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.reference }
    }
}

impl<'a, T> Deref for DataMutRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.reference }
    }
}

impl<'a, T> DerefMut for DataMutRef<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.reference }
    }
}
