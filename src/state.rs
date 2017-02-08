use std::collections::{HashMap, HashSet};
use std::any::{Any, TypeId};
use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
use std::ops::{Deref, DerefMut};
use webplatform::{self, HtmlNode};

use {AppContext, EventType, Renderable, Node};

pub struct Handler<'doc> {
    el: Option<String>,
    event_type: EventType,
    event_handler: Rc<Fn(webplatform::Event<'doc>, usize) + 'doc>,
    registered_nodes: RefCell<Vec<Rc<HtmlNode<'doc>>>>,
}

pub struct Binding<'doc> {
    pub node: Rc<HtmlNode<'doc>>,
    component: Box<Renderable>,
    // FIXME: Store selector and handlers, and blindly reapply handlers after rerender until we can patch DOM more conservatively
    handlers: Vec<Handler<'doc>>,
}

impl<'doc> Binding<'doc> {
    pub fn new<R: 'static + Renderable>(component: R, node: Rc<HtmlNode<'doc>>) -> Binding<'doc> {
        Binding {
            component: Box::new(component),
            node: node,
            handlers: vec![],
        }
    }

    pub fn add_handler(&mut self,
                       event_type: EventType,
                       el: Option<String>,
                       event_handler: Rc<Fn(webplatform::Event<'doc>, usize) + 'doc>,
                       registered_nodes: Vec<Rc<HtmlNode<'doc>>>) {
        let handler = Handler {
            el: el,
            event_type: event_type,
            event_handler: event_handler,
            registered_nodes: RefCell::new(registered_nodes),
        };
        self.handlers.push(handler);
    }


    pub fn component<R>(&self) -> &R
        where R: Renderable
    {
        self.component.downcast_ref().unwrap()
    }

    pub fn component_mut<R>(&mut self) -> &mut R
        where R: Renderable
    {
        self.component.downcast_mut().unwrap()
    }
}

// Map data_id to data
type DataStore = HashMap<TypedKey, Box<Any>>;

// Map view_id to binding
type BindingStore<'doc> = HashMap<TypedKey, Rc<RefCell<Binding<'doc>>>>;

// Map data_id to view_ids that are observing said data
type ObserverStore = HashMap<TypedKey, HashSet<TypedKey>>;

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

    pub fn data<T: 'static>(&self, key: &str) -> Option<DataRef<T>> {
        let data_id = TypedKey::new::<T>(key);
        let borrowed_state = self.state.borrow();
        if !borrowed_state.contains_key(&data_id) {
            return None;
        }
        let owned_ref = Ref::map(borrowed_state, |state| {
            let entry = state.get(&data_id).unwrap();
            entry.downcast_ref().unwrap()
        });
        Some(DataRef {
            reference: &*owned_ref,
            _owner: owned_ref,
        })
    }

    pub fn data_mut<T: 'static>(&self, key: &str) -> Option<DataMutRef<T>> {
        // Look up observers, and enqueue them for re-render
        let data_id = TypedKey::new::<T>(key);
        let borrowed_state = self.state.borrow_mut();
        if !borrowed_state.contains_key(&data_id) {
            return None;
        }

        {
            let observers = self.observers.borrow();
            if let Some(partition_observers) = observers.get(&data_id) {
                let mut queue = self.render_queue.borrow_mut();
                for observer in partition_observers {
                    queue.push(observer.clone());
                }
            }
        }


        let mut owned_ref = RefMut::map(borrowed_state, |mut state| {
            let mut entry = state.get_mut(&data_id).expect("Failed to get mutable state");
            entry.downcast_mut::<T>().unwrap()
        });
        Some(DataMutRef {
            reference: &mut *owned_ref,
            _owner: owned_ref,
        })
    }

    // TODO: figure out if I want this to be safe to call from event handlers
    //    if so, this needs to have the same observer rendering logic as `data_mut`
    pub fn data_set<T: 'static>(&self, key: &str, data: T) {
        let data_id = TypedKey::new::<T>(key);
        let mut borrowed_state = self.state.borrow_mut();
        borrowed_state.insert(data_id, Box::new(data));
    }

    pub fn insert_binding<R: 'static + Renderable>(&self,
                                                   key: &str,
                                                   component: R,
                                                   node: Rc<HtmlNode<'doc>>)
                                                   -> Rc<RefCell<Binding<'doc>>> {
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
        let mut partition = observers.entry(data_id).or_insert_with(|| HashSet::new());
        partition.insert(view_id);
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

            let render_node = Node::new(Rc::new(self.clone()), binding.node.clone());
            let app_context = AppContext::new(Rc::new(self.clone()), Some(view_id.clone()));
            binding.node.html_patch(&component.render(&render_node, &app_context));

            // Attach any event handlers.
            // Since we patched the DOM, we need to reattach any event handlers
            // to any new nodes that might have been rendered
            for handler in &binding.handlers {
                if let Some(ref el) = handler.el {
                    let nodes = binding.node.element_query_all(&el);
                    let rc_nodes: Vec<_> = nodes.into_iter().map(Rc::new).collect();
                    let mut registered_nodes = handler.registered_nodes.borrow_mut();

                    for (i, node) in rc_nodes.iter().enumerate() {
                        let rc_node = Rc::new(node);
                        if registered_nodes.contains(&rc_node) {
                            continue;
                        }
                        let f = handler.event_handler.clone();
                        rc_node.on(handler.event_type.name(), move |event| f(event, i));
                    }
                    println!("On handlers REregistered for nodes: {:?}", &rc_nodes);
                    *registered_nodes = rc_nodes;
                }
            }
        }
        queue.clear();
    }

    fn clone(&self) -> AppState<'doc> {
        AppState {
            bindings: self.bindings.clone(),
            state: self.state.clone(),
            observers: self.observers.clone(),
            render_queue: self.render_queue.clone(),
        }
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
