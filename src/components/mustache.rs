use mustache::{encoder, Data, Template};
use rustc_serialize::Encodable;
use ::std::collections::HashMap;
use super::{Component, Renderable, Properties};

impl<D: 'static + Encodable> Renderable for Component<D, Template> {
    fn props(&self) -> &[&'static str] {
        &self.props
    }

    fn render<'doc>(&self, props: Properties) -> String {
        let mut data = encoder::encode(&self.data).unwrap_or_else(|err| {
            println!("Failed to encode component data. {}. Using empty hash", err);
            Data::Map(HashMap::new())
        });

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
