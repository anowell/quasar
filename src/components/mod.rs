use std::collections::HashMap;

#[cfg(feature = "mustache")]
mod mustache;

pub type Properties = HashMap<&'static str, String>;

pub trait Renderable {
    /// Register interest in specific element properties
    ///
    /// Any property names returned will be queried for
    /// there value and added to `props` before calling `render`
    fn props(&self) -> &[&'static str] { &[] }

    /// Render the component to a string
    ///
    /// `props` contains key-value pairs for any keys
    /// that were returned when calling `props`
    fn render(&self, props: Properties) -> String;
}

/// Component for templating
pub struct Component<D, T> {
    pub data: D,
    pub template: T,
    pub props: Vec<&'static str>,
}

