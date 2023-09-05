use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::error::ProseMirrorError;
use crate::utils::push_front;
use crate::ProseMirror;

mod text;

pub trait Plugin {
    fn render(
        &self,
        node: &Value,
        plugins: &HashMap<String, Box<dyn Plugin>>,
    ) -> Result<String, ProseMirrorError>;
}

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
        println!("{:?}", attrs);
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
                attrs.map(Tag::create_attrs).unwrap_or_default()
            )
        } else {
            format!(
                "<{}{}>",
                self.name,
                attrs
                    .map(Tag::create_attrs)
                    .map(|s| push_front(s, " "))
                    .unwrap_or_default()
            )
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

macro_rules! define_tag_plugin {
    ($struct_name:ident, $type_name:expr, $tag_name:expr, $is_self_closing:expr) => {
        pub struct $struct_name;

        impl Plugin for $struct_name {
            fn render(
                &self,
                node: &Value,
                plugins: &HashMap<String, Box<dyn Plugin>>,
            ) -> Result<std::string::String, ProseMirrorError> {
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

            pub fn register(prosemirror: &mut ProseMirror) {
                prosemirror.add_plugin($type_name, $struct_name::new());
            }
        }
    };
}

define_tag_plugin!(DocPlugin, "doc", "div", false);
define_tag_plugin!(ParagraphPlugin, "paragraph", "p", false);
define_tag_plugin!(ImagePlugin, "image", "img", true);

#[cfg(test)]
mod tests {
    use crate::plugins::text::TextPlugin;

    use super::*;
    use serde_json::json;

    #[test]
    fn it_throws_error_on_unknown_type() {
        let content = json!({"type":"doc","content":[]});
        let prose_mirror = ProseMirror::new(content);

        assert_eq!(
            prose_mirror.render().unwrap_err(),
            ProseMirrorError::TypeNotFound {
                type_name: Some("doc".to_owned())
            }
        );
    }

    #[test]
    fn it_renders_paragraph_plugin() {
        let content = json!({"type":"doc","content":[{"type":"paragraph","content":[{"text":"This is a comment on the Leafs thread","type":"text"}]}]});
        let mut prose_mirror = ProseMirror::new(content);

        DocPlugin::register(&mut prose_mirror);
        ParagraphPlugin::register(&mut prose_mirror);
        TextPlugin::register(&mut prose_mirror);

        assert_eq!(
            prose_mirror.render().unwrap(),
            "<div><p>This is a comment on the Leafs thread</p></div>".to_string()
        );
    }

    #[test]
    fn it_renders_paragraph_plugin_with_attrs() {
        let content = json!({"type":"doc","content":[{"type":"paragraph","attrs": {"class": "test"}, "content":[{"text":"This is a comment on the Leafs thread","type":"text"}]}]});
        let mut prose_mirror = ProseMirror::new(content);

        DocPlugin::register(&mut prose_mirror);
        ParagraphPlugin::register(&mut prose_mirror);
        TextPlugin::register(&mut prose_mirror);

        assert_eq!(
            prose_mirror.render().unwrap(),
            "<div><p class=\"test\">This is a comment on the Leafs thread</p></div>".to_string()
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

        let mut prose_mirror = ProseMirror::new(content);

        ImagePlugin::register(&mut prose_mirror);

        assert_eq!(prose_mirror.render().unwrap(), "<img alt=\"PAPI SIGNS EXTENSION 😏\" src=\"https://pbs.twimg.com/media/F4PrVzTXwAAADiF?format=jpg&name=large\" title=\"\" />".to_owned());
    }
}
