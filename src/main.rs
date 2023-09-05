use error::ProseMirrorError;
use plugins::Plugin;
use serde_json::{Map, Value};
use std::collections::HashMap;

mod error;
mod plugins;
mod utils;

pub struct ProseMirror {
    content: Value,
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl ProseMirror {
    pub fn new(content: Value) -> Self {
        Self {
            content,
            plugins: HashMap::new(),
        }
    }

    pub fn add_plugin(&mut self, node_type: &str, plugin: Box<dyn Plugin>) {
        self.plugins.insert(node_type.to_string(), plugin);
    }

    pub fn render(&self) -> Result<String, ProseMirrorError> {
        let type_name = self.content.get("type");
        if let Some(node_type) = type_name {
            if let Some(plugin) = self.plugins.get(node_type.as_str().unwrap()) {
                return Ok(plugin.render(&self.content, &self.plugins)?);
            }
        }
        Err(ProseMirrorError::TypeNotFound {
            type_name: self
                .content
                .get("type")
                .map(|t| t.as_str().unwrap_or_default().to_string()),
        })
    }
}

fn main() {}
