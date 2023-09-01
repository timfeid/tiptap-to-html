extern crate serde_json;

use error::TiptapError;
use serde_json::{Map, Value};
use std::collections::HashMap;

mod error;

pub struct Tag {
    name: &'static str,
    is_self_closing: bool,
}

impl Tag {
    pub fn new(name: &'static str, is_self_closing: bool) -> Self {
        Self {
            name,
            is_self_closing,
        }
    }

    fn create_attrs(attrs: &Map<String, Value>) -> String {
        let mut attr_strs = vec![];

        for (key, value) in attrs.iter() {
            let value_str = match value {
                Value::Null => "".to_string(),
                Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            attr_strs.push(format!("{}=\"{}\"", key, value_str));
        }

        attr_strs.join(" ")
    }

    pub fn render(&self, output: String, node: &Value) -> String {
        return format!(
            "{}{}{}",
            self.render_opening(node.get("attrs").and_then(|attrs| attrs.as_object())),
            output,
            self.render_closing()
        );
    }

    pub fn render_opening(&self, attrs: Option<&Map<String, Value>>) -> String {
        if self.is_self_closing {
            format!(
                "<{} {} />",
                self.name,
                attrs
                    .map(|attrs| Tag::create_attrs(attrs))
                    .unwrap_or_default()
            )
        } else {
            format!("<{}>", self.name)
        }
    }

    pub fn render_closing(&self) -> String {
        if self.is_self_closing {
            String::new()
        } else {
            format!("</{}>", self.name)
        }
    }
}

// Define a trait for plugins
pub trait Plugin {
    fn render(
        &self,
        node: &Value,
        plugins: &HashMap<String, Box<dyn Plugin>>,
    ) -> Result<String, TiptapError>;
}

// Create a Tiptap struct
pub struct Tiptap {
    content: Value,
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl Tiptap {
    pub fn new(content: Value) -> Self {
        Self {
            content,
            plugins: HashMap::new(),
        }
    }

    pub fn add_plugin(&mut self, node_type: &str, plugin: Box<dyn Plugin>) {
        self.plugins.insert(node_type.to_string(), plugin);
    }

    pub fn render(&self) -> Result<String, TiptapError> {
        let type_name = self.content.get("type");
        if let Some(node_type) = type_name {
            if let Some(plugin) = self.plugins.get(node_type.as_str().unwrap()) {
                return Ok(plugin.render(&self.content, &self.plugins)?);
            }
        }
        Err(TiptapError::TypeNotFound {
            type_name: self
                .content
                .get("type")
                .map(|t| t.as_str().unwrap_or_default().to_string()),
        })
    }
}

pub struct TextPlugin;
impl Plugin for TextPlugin {
    fn render(
        &self,
        node: &Value,
        plugins: &HashMap<String, Box<dyn Plugin>>,
    ) -> Result<String, TiptapError> {
        let mut output = String::new();
        if let Some(text) = node.get("text") {
            output.push_str(text.as_str().unwrap());
        }

        Ok(output)
    }
}

macro_rules! define_tag_plugin {
    ($struct_name:ident, $type_name:expr, $tag_name:expr, $is_self_closing:expr) => {
        pub struct $struct_name;

        impl Plugin for $struct_name {
            fn render(
                &self,
                node: &Value,
                plugins: &HashMap<String, Box<dyn Plugin>>,
            ) -> Result<std::string::String, TiptapError> {
                let mut output = String::new();
                if let Some(content) = node.get("content") {
                    for child_node in content.as_array().unwrap() {
                        if let Some(child_node_type) = child_node.get("type") {
                            if let Some(plugin) = plugins.get(child_node_type.as_str().unwrap()) {
                                output.push_str(&plugin.render(child_node, plugins)?);
                            }
                        }
                    }
                }

                let tag = self.get_tag();
                Ok(tag.render(output, node))
            }
        }

        impl $struct_name {
            fn get_tag(&self) -> Tag {
                Tag::new($tag_name, $is_self_closing)
            }

            pub fn new() -> Box<dyn Plugin> {
                Box::new(Self)
            }

            pub fn type_name() -> &'static str {
                $type_name
            }

            pub fn register(tiptap: &mut Tiptap) {
                tiptap.add_plugin($type_name, $struct_name::new());
            }
        }
    };
}

define_tag_plugin!(DocPlugin, "doc", "div", false);
define_tag_plugin!(ParagraphPlugin, "paragraph", "p", false);
define_tag_plugin!(ImagePlugin, "image", "img", true);

impl TextPlugin {
    pub fn new() -> Box<dyn Plugin> {
        Box::new(Self)
    }

    pub fn register(tiptap: &mut Tiptap) {
        tiptap.add_plugin("text", TextPlugin::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_throws_error_on_unknown_type() {
        let content = json!({"type":"doc","content":[]});
        let tiptap = Tiptap::new(content);

        assert_eq!(
            tiptap.render().unwrap_err(),
            TiptapError::TypeNotFound {
                type_name: Some("doc".to_owned())
            }
        );
    }

    #[test]
    fn render() {
        let content = json!({"type":"doc","content":[{"type":"paragraph","content":[{"text":"This is a comment on the Leafs thread","type":"text"}]}]});
        let mut tiptap = Tiptap::new(content);

        // Register plugins
        DocPlugin::register(&mut tiptap);
        ParagraphPlugin::register(&mut tiptap);
        TextPlugin::register(&mut tiptap);

        assert_eq!(
            tiptap.render().unwrap(),
            "<div><p>This is a comment on the Leafs thread</p></div>".to_string()
        );
    }

    #[test]
    fn image() {
        let content = json!({
          "type": "image",
          "attrs": {
            "alt": "PAPI SIGNS EXTENSION 😏",
            "src": "https://pbs.twimg.com/media/F4PrVzTXwAAADiF?format=jpg&name=large",
            "title": null
          }
        });

        let mut tiptap = Tiptap::new(content);

        ImagePlugin::register(&mut tiptap);

        assert_eq!(tiptap.render().unwrap(), "<img alt=\"PAPI SIGNS EXTENSION 😏\" src=\"https://pbs.twimg.com/media/F4PrVzTXwAAADiF?format=jpg&name=large\" title=\"\" />".to_owned());
    }
}

fn main() {
    // Your main logic here
}
