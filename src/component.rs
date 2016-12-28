use mustache::{self, encoder, Template};
use rustc_serialize::Encodable;
use webplatform::HtmlNode;
use std::collections::HashMap;

pub struct Component<Data> {
    pub template: Template,
    pub data: Data,
    pub props: Vec<&'static str>,
}

impl <Data: Encodable> Component<Data> {
    // TODO: hide from docs - pub{crate}
    pub fn render<'doc>(&self, node: &HtmlNode<'doc>) -> String {
        let mut data = encoder::encode(&self.data).expect("Failed to encode component data");
        let mut output = Vec::new();

        // Augment the scope data with 'props'
        let mut props = HashMap::new();
        for prop in &self.props {
            let val = node.prop_get_str(prop);
            props.insert(prop.to_string(), mustache::Data::StrVal(val));
        }

        match data {
            mustache::Data::Map(ref mut map) => {
                map.insert("props".to_string(), mustache::Data::Map(props));
            }
            _ => panic!("Unexpected data encoding")
        }

        self.template.render_data(&mut output, &data).expect("failed to render component");
        String::from_utf8_lossy(&output).into_owned()
    }
}