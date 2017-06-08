use mustache::{encoder, Data, Template};
use rustc_serialize::Encodable;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use quasar::{Node, AppContext};
use quasar::Renderable;

/// Helper type for runtime templating
#[derive(Debug)]
pub struct RuntimeComponent<D, T> {
    pub data: D,
    pub template: T,
    pub props: Vec<&'static str>,
}

impl<D, T> Deref for RuntimeComponent<D, T> {
    type Target = D;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<D, T> DerefMut for RuntimeComponent<D, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<D: 'static + Encodable> Renderable for RuntimeComponent<D, Template> {
    fn render<'doc>(&self, node: &Node, _app: &AppContext) -> String {
        let mut data = encoder::encode(&self.data).unwrap_or_else(|err| {
            println!("Failed to encode component data. {}. Using empty hash", err);
            Data::Map(HashMap::new())
        });

        let props = node.get_properties(&self.props);

        match data {
            Data::Map(ref mut map) => {
                let mustache_props = props.into_iter()
                    .map(|(k, v)| (k.to_string(), Data::StrVal(v)))
                    .collect();
                map.insert("props".to_string(), Data::Map(mustache_props));
            }
            _ => panic!("Unexpected data encoding"),
        }

        let mut output = Vec::new();
        self.template.render_data(&mut output, &data).expect("failed to render component");
        String::from_utf8_lossy(&output).into_owned()
    }
}

