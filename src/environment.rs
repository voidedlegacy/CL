
use std::collections::HashMap;
use crate::node::{Node, NodeValue};

#[derive(Debug)]
pub struct Environment {
    pub parent: Option<Box<Environment>>,
    pub bind: HashMap<String, Node>,
}

impl Environment {
    pub fn new(parent: Option<Box<Environment>>) -> Self {
        Environment {
            parent,
            bind: HashMap::new(),
        }
    }

    pub fn set(&mut self, id: &Node, value: Node) -> Result<(), String> {
        if let Some(NodeValue::Symbol(ref id_str)) = id.value {
            self.bind.insert(id_str.clone(), value);
            Ok(())
        } else {
            Err("Invalid ID type".to_string())
        }
    }

    pub fn get(&self, id: &Node) -> Option<&Node> {
        if let Some(NodeValue::Symbol(ref id_str)) = id.value {
            self.bind.get(id_str)
        } else {
            None
        }
    }
}
