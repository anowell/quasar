use std::collections::HashMap;
use downcast_rs::Downcast;
use {AppContext, Node};

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
