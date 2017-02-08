use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use downcast_rs::Downcast;
use {AppContext, Node};

#[cfg(feature = "mustache")]
mod mustache;

pub type Properties = HashMap<&'static str, String>;

pub trait Renderable: Downcast {
    /// Render the component to a string
    ///
    /// Rendering may include inspecting Node properties, inner HTML.
    /// Reading state from `app` will register this `Renderable` as
    /// an observer of said global state which will cause rerendering
    /// when that state changes.
    ///
    /// Additionally, it is possible to attach additional handlers directly to the node
    /// making it possible to build templating that adds handlers automatically
    fn render(&self, node: &Node, app: &AppContext) -> String;
}

impl_downcast!(Renderable);

impl<T> Renderable for T
    where T: ::std::fmt::Display + 'static
{
    fn render<'doc>(&self, _node: &Node, _app: &AppContext) -> String {
        self.to_string()
    }
}


/// Component for templating
#[derive(Debug)]
pub struct Component<D, T> {
    pub data: D,
    pub template: T,
    pub props: Vec<&'static str>,
}

impl<D, T> Deref for Component<D, T> {
    type Target = D;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<D, T> DerefMut for Component<D, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
