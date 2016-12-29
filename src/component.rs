use mustache::{self, encoder, Template};
use rustc_serialize::Encodable;
use std::collections::HashMap;

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

/// Component for Mustache templating
pub struct Component<D: Encodable> {
    pub data: D,
    pub template: Template,
    pub props: Vec<&'static str>,
}

impl <D: Encodable> Renderable for Component<D> {
    fn props(&self) -> &[&'static str] {
        &self.props
    }

    fn render<'doc>(&self, props: Properties) -> String {
        let mut data = encoder::encode(&self.data).expect("Failed to encode component data");
        match data {
            mustache::Data::Map(ref mut map) => {
                let mustache_props = props.into_iter().map(|(k,v)| {
                    (k.to_string(), mustache::Data::StrVal(v))
                }).collect();
                map.insert("props".to_string(), mustache::Data::Map(mustache_props));
            }
            _ => panic!("Unexpected data encoding")
        }

        let mut output = Vec::new();
        self.template.render_data(&mut output, &data).expect("failed to render component");
        String::from_utf8_lossy(&output).into_owned()
    }
}