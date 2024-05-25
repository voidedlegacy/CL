use crate::environment::Environment;
use crate::lexer::{lex, token_string_equalp, Token};
use crate::node::{Node, NodeType, NodeValue};

#[derive(Debug)]
pub struct ParsingContext {
    pub types: Environment,
    pub variables: Environment,
}

impl ParsingContext {
    pub fn new() -> Self {
        let mut types = Environment::new(None);
        types.set(&Node::from_symbol("integer"), Node::from_integer(0)).unwrap();
        ParsingContext {
            types,
            variables: Environment::new(None),
        }
    }
}

pub fn parse_integer(token: &Token, source: &str) -> Result<Node, ()> {
    let token_str = &source[token.beginning..token.end];
    if token_str == "0" {
        Ok(Node::from_integer(0))
    } else if let Ok(value) = token_str.parse::<i64>() {
        Ok(Node::from_integer(value))
    } else {
        Err(())
    }
}

pub fn parse_expr(context: &mut ParsingContext, source: &str, end: &mut usize) -> Result<Node, String> {
    let mut current_token = Token::new(*end, *end);
    let mut result = Node::new(NodeType::None, None);

    while lex(source, &mut current_token).is_ok() {
        *end = current_token.end;
        let token_length = current_token.end - current_token.beginning;
        if token_length == 0 {
            break;
        }

        if let Ok(integer_node) = parse_integer(&current_token, source) {
            result = integer_node;
            current_token.beginning = current_token.end;
            if lex(source, &mut current_token).is_err() {
                return Ok(result);
            }
            *end = current_token.end;
        } else {
            let symbol_node = Node::from_symbol_buffer(&source[current_token.beginning..current_token.end]);
            current_token.beginning = current_token.end;
            if lex(source, &mut current_token).is_err() {
                return Ok(result);
            }
            *end = current_token.end;

            if token_string_equalp(":", &current_token, source) {
                current_token.beginning = current_token.end;
                if lex(source, &mut current_token).is_err() {
                    return Ok(result);
                }
                *end = current_token.end;

                let expected_type_symbol = Node::from_symbol_buffer(&source[current_token.beginning..current_token.end]);
                if let Some(_) = context.types.get(&expected_type_symbol) {
                    let mut var_decl = Node::new(
                        NodeType::VariableDeclaration,
                        Some(NodeValue::VariableDeclaration {
                            name: symbol_node.to_string(),
                            var_type: expected_type_symbol.to_string(),
                        }),
                    );

                    if lex(source, &mut current_token).is_ok() && token_string_equalp("=", &current_token, source) {
                        current_token.beginning = current_token.end;
                        if lex(source, &mut current_token).is_err() {
                            return Ok(result);
                        }
                        *end = current_token.end;

                        let value_node = parse_expr(context, source, end)?;
                        var_decl.node_type = NodeType::VariableDeclarationInitialized;
                        var_decl.add_child(value_node);
                        result = var_decl;
                    } else {
                        result = var_decl;
                    }
                    return Ok(result);
                } else {
                    let err = format!(
                        "Invalid type within variable declaration: {}",
                        &source[current_token.beginning..current_token.end]
                    );
                    return Err(err);
                }
            }

            if token_string_equalp(":=", &current_token, source) {
                current_token.beginning = current_token.end;
                if lex(source, &mut current_token).is_err() {
                    return Ok(result);
                }
                *end = current_token.end;

                let value_node = parse_expr(context, source, end)?;
                let var_assign = Node::new(
                    NodeType::VariableAssignment,
                    Some(NodeValue::VariableAssignment {
                        name: symbol_node.to_string(),
                        value: Box::new(value_node),
                    }),
                );

                result = var_assign;
                return Ok(result);
            }

            if token_string_equalp("defun", &current_token, source) {
                current_token.beginning = current_token.end;
                if lex(source, &mut current_token).is_err() {
                    return Ok(result);
                }
                *end = current_token.end;

                if lex(source, &mut current_token).is_err() {
                    return Ok(result);
                }
                let func_name_node = Node::from_symbol_buffer(&source[current_token.beginning..current_token.end]);
                current_token.beginning = current_token.end;
                if lex(source, &mut current_token).is_err() {
                    return Ok(result);
                }
                *end = current_token.end;

                let mut params = Vec::new();
                if token_string_equalp("(", &current_token, source) {
                    current_token.beginning = current_token.end;
                    while lex(source, &mut current_token).is_ok() && !token_string_equalp(")", &current_token, source) {
                        let param_name = Node::from_symbol_buffer(&source[current_token.beginning..current_token.end]);
                        current_token.beginning = current_token.end;

                        if lex(source, &mut current_token).is_err() {
                            return Ok(result);
                        }
                        *end = current_token.end;

                        if token_string_equalp(":", &current_token, source) {
                            current_token.beginning = current_token.end;
                            if lex(source, &mut current_token).is_err() {
                                return Ok(result);
                            }
                            *end = current_token.end;

                            let param_type = Node::from_symbol_buffer(&source[current_token.beginning..current_token.end]);
                            params.push((param_name.to_string(), param_type.to_string()));
                        }

                        current_token.beginning = current_token.end;
                    }
                    if lex(source, &mut current_token).is_err() {
                        return Ok(result);
                    }
                    *end = current_token.end;
                }

                let mut return_type = "void".to_string();
                if token_string_equalp(":", &current_token, source) {
                    current_token.beginning = current_token.end;
                    if lex(source, &mut current_token).is_err() {
                        return Ok(result);
                    }
                    *end = current_token.end;

                    return_type = Node::from_symbol_buffer(&source[current_token.beginning..current_token.end]).to_string();
                }

                let mut body = Vec::new();
                if token_string_equalp("{", &current_token, source) {
                    current_token.beginning = current_token.end;
                    while lex(source, &mut current_token).is_ok() && !token_string_equalp("}", &current_token, source) {
                        let stmt = parse_expr(context, source, end)?;
                        body.push(stmt);
                        current_token.beginning = current_token.end;
                    }
                }

                let func_def = Node::new(
                    NodeType::FunctionDefinition,
                    Some(NodeValue::FunctionDefinition {
                        name: func_name_node.to_string(),
                        params,
                        return_type,
                        body,
                    }),
                );

                result = func_def;
                return Ok(result);
            }

            println!("Unrecognized token: ");
            crate::lexer::print_token(&current_token, source);
            println!();

            return Err("Syntax error".to_string());
        }

        println!("Intermediate node: ");
        result.print(0);
        println!();
    }

    Ok(result)
}
