use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    None,
    Integer,
    Symbol,
    VariableDeclaration,
    VariableDeclarationInitialized,
    VariableAssignment,
    FunctionDefinition,
    Program,
}

#[derive(Debug, Clone)]
pub enum NodeValue {
    Integer(i64),
    Symbol(String),
    VariableDeclaration { name: String, var_type: String },
    VariableAssignment { name: String, value: Box<Node> },
    FunctionDefinition {
        name: String,
        params: Vec<(String, String)>,
        return_type: String,
        body: Vec<Node>,
    },
}

#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub value: Option<NodeValue>,
    pub children: Vec<Node>,
    pub next_child: Option<Box<Node>>,
}

impl Node {
    pub fn new(node_type: NodeType, value: Option<NodeValue>) -> Self {
        Node {
            node_type,
            value,
            children: Vec::new(),
            next_child: None,
        }
    }

    pub fn add_child(&mut self, new_child: Node) {
        if let Some(last_child) = self.children.last_mut() {
            let mut last_child = last_child;
            while let Some(ref mut next) = last_child.next_child {
                last_child = next;
            }
            last_child.next_child = Some(Box::new(new_child));
        } else {
            self.children.push(new_child);
        }
    }

    pub fn compare(a: &Node, b: &Node) -> bool {
        if a.node_type != b.node_type {
            return false;
        }
        match a.node_type {
            NodeType::None => true,
            NodeType::Integer => {
                if let (Some(NodeValue::Integer(a_val)), Some(NodeValue::Integer(b_val))) = (&a.value, &b.value) {
                    a_val == b_val
                } else {
                    false
                }
            }
            NodeType::Symbol => {
                if let (Some(NodeValue::Symbol(ref a_val)), Some(NodeValue::Symbol(ref b_val))) = (&a.value, &b.value) {
                    a_val == b_val
                } else {
                    false
                }
            }
            _ => unimplemented!("Node type comparison not implemented for {:?}", a.node_type),
        }
    }

    pub fn from_integer(value: i64) -> Self {
        Node::new(NodeType::Integer, Some(NodeValue::Integer(value)))
    }

    pub fn from_symbol(symbol: &str) -> Self {
        Node::new(NodeType::Symbol, Some(NodeValue::Symbol(symbol.to_string())))
    }

    pub fn from_symbol_buffer(buffer: &str) -> Self {
        Node::new(NodeType::Symbol, Some(NodeValue::Symbol(buffer.to_string())))
    }

    pub fn print(&self, indent_level: usize) {
        for _ in 0..indent_level {
            print!(" ");
        }
        match &self.node_type {
            NodeType::None => println!("NONE"),
            NodeType::Integer => {
                if let Some(NodeValue::Integer(value)) = &self.value {
                    println!("INT:{}", value);
                }
            }
            NodeType::Symbol => {
                print!("SYM");
                if let Some(NodeValue::Symbol(ref symbol)) = &self.value {
                    println!(":{}", symbol);
                }
            }
            NodeType::VariableDeclaration => {
                if let Some(NodeValue::VariableDeclaration { name, var_type }) = &self.value {
                    println!("VAR DECLARATION: {} : {}", name, var_type);
                }
            }
            NodeType::VariableDeclarationInitialized => {
                if let Some(NodeValue::VariableDeclaration { name, var_type }) = &self.value {
                    println!("VAR DECLARATION INITIALIZED: {} : {}", name, var_type);
                }
                for child in &self.children {
                    child.print(indent_level + 4);
                }
            }
            NodeType::VariableAssignment => {
                if let Some(NodeValue::VariableAssignment { name, value }) = &self.value {
                    println!("VAR ASSIGNMENT: {} := {}", name, value);
                }
                for child in &self.children {
                    child.print(indent_level + 4);
                }
            }
            NodeType::FunctionDefinition => {
                if let Some(NodeValue::FunctionDefinition { name, params, return_type, body }) = &self.value {
                    println!("FUNCTION DEFINITION: {} (", name);
                    for (param_name, param_type) in params {
                        print!("{}: {}, ", param_name, param_type);
                    }
                    println!("): {}", return_type);
                    for stmt in body {
                        stmt.print(indent_level + 4);
                    }
                }
            }
            NodeType::Program => println!("PROGRAM"),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.node_type {
            NodeType::None => write!(f, "NONE"),
            NodeType::Integer => {
                if let Some(NodeValue::Integer(value)) = &self.value {
                    write!(f, "INT:{}", value)
                } else {
                    write!(f, "INT: <no value>")
                }
            }
            NodeType::Symbol => {
                if let Some(NodeValue::Symbol(symbol)) = &self.value {
                    write!(f, "SYM:{}", symbol)
                } else {
                    write!(f, "SYM: <no value>")
                }
            }
            NodeType::VariableDeclaration => {
                if let Some(NodeValue::VariableDeclaration { name, var_type }) = &self.value {
                    write!(f, "VAR DECLARATION: {} : {}", name, var_type)
                } else {
                    write!(f, "VAR DECLARATION: <no value>")
                }
            }
            NodeType::VariableDeclarationInitialized => {
                if let Some(NodeValue::VariableDeclaration { name, var_type }) = &self.value {
                    write!(f, "VAR DECLARATION INITIALIZED: {} : {}", name, var_type)
                } else {
                    write!(f, "VAR DECLARATION INITIALIZED: <no value>")
                }
            }
            NodeType::VariableAssignment => {
                if let Some(NodeValue::VariableAssignment { name, value }) = &self.value {
                    write!(f, "VAR ASSIGNMENT: {} := {}", name, value)
                } else {
                    write!(f, "VAR ASSIGNMENT: <no value>")
                }
            }
            NodeType::FunctionDefinition => {
                if let Some(NodeValue::FunctionDefinition { name, params, return_type, body }) = &self.value {
                    write!(f, "FUNCTION DEFINITION: {} (", name)?;
                    for (param_name, param_type) in params {
                        write!(f, "{}: {}, ", param_name, param_type)?;
                    }
                    write!(f, "): {}", return_type)?;
                    for stmt in body {
                        write!(f, "\n{}", stmt)?;
                    }
                    Ok(())
                } else {
                    write!(f, "FUNCTION DEFINITION: <no value>")
                }
            }
            NodeType::Program => write!(f, "PROGRAM"),
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        if let Some(NodeValue::Symbol(ref mut symbol)) = self.value {
            symbol.clear();
        }
    }
}
