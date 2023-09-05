use std::collections::HashMap;

use serde_json::Value;

use crate::{error::ProseMirrorError, ProseMirror};

use super::Plugin;

pub struct TextPlugin;
impl Plugin for TextPlugin {
    fn render(
        &self,
        node: &Value,
        plugins: &HashMap<String, Box<dyn Plugin>>,
    ) -> Result<String, ProseMirrorError> {
        let mut output = String::new();
        if let Some(text) = node.get("text") {
            output.push_str(text.as_str().unwrap());
        }

        Ok(output)
    }
}

impl TextPlugin {
    pub fn new() -> Box<dyn Plugin> {
        Box::new(Self)
    }

    pub fn register(prosemirror: &mut ProseMirror) {
        prosemirror.add_plugin("text", TextPlugin::new());
    }
}
