use state::{AppState, DataRef, DataMutRef, TypedKey};
use std::rc::Rc;
use webplatform;
use uuid::Uuid;

use {Queryable, Renderable, View, Node};

/// The main app object instantiated by calling `quasar::init()`
pub struct QuasarApp<'doc> {
    app: Rc<AppState<'doc>>,
}

/// Provides select access to the global `QuasarApp` object in the context of a specific `View`
pub struct AppContext<'doc> {
    app: Rc<AppState<'doc>>,
    view_id: Option<TypedKey>,
}

pub fn init<'a, 'doc: 'a>() -> QuasarApp<'a> {
    let document = webplatform::init();
    QuasarApp {
        app: Rc::new(AppState::new(document)),
    }
}

impl<'doc> AppContext<'doc> {
    #![doc(hidden)]
    pub fn new(app: Rc<AppState<'doc>>, view_id: Option<TypedKey>) -> AppContext<'doc> {
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
        if let Some(ref view_id) = self.view_id {
            self.app.add_observer(type_id, view_id.clone());
        }
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
        if let Some(ref view_id) = self.view_id {
            self.app.add_observer(type_id, view_id.clone());
        }
        self.app.data_mut(key)
    }
}



impl<'doc> QuasarApp<'doc> {
    pub fn query_all(&self, el: &str) -> Vec<Node<'doc>> {
        self.app.document.element_query_all(el).into_iter().map(|node| {
            Node::new(self.app.clone(), Rc::new(node))
        }).collect()
    }

    /// Get app data for a specific key
    pub fn data<T: 'static>(&self, key: &str) -> Option<DataRef<T>> {
        self.app.data(key)
    }

    /// Get app data for a specific key
    pub fn data_mut<T: 'static>(&mut self, key: &str) -> Option<DataMutRef<T>> {
        self.app.data_mut(key)
    }

    /// Get app data for a specific key
    pub fn data_set<T: 'static>(&mut self, key: &str, data: T) {
        self.app.data_set(key, data)
    }
}


impl<'doc> Queryable<'doc> for QuasarApp<'doc> {
    type Q = Node<'doc>;
    fn query(&self, el: &str) -> Option<Node<'doc>> {
        self.app.document.element_query(el).map(|node| {
            Node::new(self.app.clone(), Rc::new(node))
        })
    }

    fn bind<R: 'static + Renderable>(&self, el: &str, component: R) -> View<'doc, R> {
        let node = self.app.document.element_query(el).expect("querySelector found no results");
        let rc_node = Rc::new(node);

        let render_node = Node::new(self.app.clone(), rc_node.clone());
        let key = Uuid::new_v4().to_string();

        let app_context = AppContext::new(self.app.clone(), Some(TypedKey::new::<R>(&key)));
        rc_node.html_patch(&component.render(&render_node, &app_context));

        let binding = self.app.insert_binding(&key, component, rc_node.clone());

        View::new(self.app.clone(), rc_node, key, binding)
    }
}

impl<'doc> Queryable<'doc> for AppContext<'doc> {
    type Q = Node<'doc>;
    fn query(&self, el: &str) -> Option<Node<'doc>> {
        self.app.document.element_query(el).map(|node| {
            Node::new(self.app.clone(), Rc::new(node))
        })
    }

    fn bind<R: 'static + Renderable>(&self, el: &str, component: R) -> View<'doc, R> {
        let node = self.app.document.element_query(el).expect("querySelector found no results");
        let rc_node = Rc::new(node);

        let render_node = Node::new(self.app.clone(), rc_node.clone());
        let key = Uuid::new_v4().to_string();

        let app_context = AppContext::new(self.app.clone(), Some(TypedKey::new::<R>(&key)));
        rc_node.html_patch(&component.render(&render_node, &app_context));

        let binding = self.app.insert_binding(&key, component, rc_node.clone());

        View::new(self.app.clone(), rc_node, key, binding)
    }
}