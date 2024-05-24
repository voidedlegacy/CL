use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::ffi::CString;

#[derive(Debug, Clone)]
enum ErrorType {
    None,
    Arguments,
    Type,
    Generic,
    Syntax,
    Todo,
    Max,
}

#[derive(Debug, Clone)]
struct Error {
    error_type: ErrorType,
    msg: Option<String>,
}

impl Error {
    fn new(error_type: ErrorType, msg: Option<String>) -> Self {
        Error { error_type, msg }
    }
}

const OK: Error = Error {
    error_type: ErrorType::None,
    msg: None,
};

fn file_size(file: &mut File) -> io::Result<u64> {
    let original_pos = file.seek(SeekFrom::Current(0))?;
    let size = file.seek(SeekFrom::End(0))?;
    file.seek(SeekFrom::Start(original_pos))?;
    Ok(size)
}

fn file_contents(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let size = file_size(&mut file)?;
    let mut contents = String::with_capacity(size as usize);
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn print_usage(program_name: &str) {
    println!("USAGE: {} <path_to_file_to_compile>", program_name);
}

fn print_error(err: &Error) {
    if let ErrorType::None = err.error_type {
        return;
    }
    println!("ERROR: {:?}", err.error_type);
    if let Some(ref msg) = err.msg {
        println!("     : {}", msg);
    }
}

#[derive(Debug, Clone)]
struct Token<'a> {
    beginning: &'a str,
    end: &'a str,
}

impl<'a> Token<'a> {
    fn new() -> Self {
        Token {
            beginning: "",
            end: "",
        }
    }
}

fn lex<'a>(source: &'a str, token: &mut Token<'a>) -> Result<(), Error> {
    if source.is_empty() {
        return Err(Error::new(
            ErrorType::Arguments,
            Some("Cannot lex empty source.".to_string()),
        ));
    }
    let whitespace = " \r\n";
    let delimiters = " \r\n,():";

    let beginning = source.find(|c| !whitespace.contains(c)).unwrap_or(0);
    let end = beginning + source[beginning..]
        .find(|c| delimiters.contains(c))
        .unwrap_or(source.len() - beginning);

    token.beginning = &source[beginning..];
    token.end = &source[beginning..end];

    if token.end == token.beginning {
        token.end = &source[beginning..beginning + 1];
    }

    if beginning >= source.len() {
        return Err(Error::new(ErrorType::None, None));
    }

    Ok(())
}

fn print_token(token: &Token) {
    println!("{}", token.beginning);
}

#[derive(Debug, Clone, PartialEq)]
enum NodeType {
    None,
    Integer,
    Symbol,
    VariableDeclaration,
    VariableDeclarationInitialized,
    BinaryOperator,
    Program,
    Max,
}

#[derive(Debug, Clone)]
struct Node {
    node_type: NodeType,
    value: Option<i64>,
    symbol: Option<String>,
    children: Vec<Box<Node>>,
}

impl Node {
    fn new(node_type: NodeType) -> Self {
        Node {
            node_type,
            value: None,
            symbol: None,
            children: vec![],
        }
    }
}

fn node_add_child(parent: &mut Node, new_child: Node) {
    parent.children.push(Box::new(new_child));
}

fn node_compare(a: &Node, b: &Node) -> bool {
    if a.node_type != b.node_type {
        return false;
    }
    match a.node_type {
        NodeType::None => b.node_type == NodeType::None,
        NodeType::Integer => a.value == b.value,
        NodeType::Program => {
            // TODO: Compare two programs.
            println!("TODO: Compare two programs.");
            false
        }
        _ => false,
    }
}

fn print_node(node: &Node, indent_level: usize) {
    for _ in 0..indent_level {
        print!(" ");
    }
    match node.node_type {
        NodeType::None => println!("NONE"),
        NodeType::Integer => println!("INT:{:?}", node.value),
        NodeType::Symbol => println!("SYM:{}", node.symbol.as_deref().unwrap_or("")),
        NodeType::VariableDeclaration => println!("VARIABLE DECLARATION"),
        NodeType::VariableDeclarationInitialized => println!("VARIABLE DECLARATION INITIALIZED"),
        NodeType::BinaryOperator => println!("BINARY OPERATOR"),
        NodeType::Program => println!("PROGRAM"),
        _ => println!("UNKNOWN"),
    }
    for child in &node.children {
        print_node(child, indent_level + 4);
    }
}

fn parse_integer(token: &Token, node: &mut Node) -> bool {
    if let Ok(value) = token.beginning.parse::<i64>() {
        node.node_type = NodeType::Integer;
        node.value = Some(value);
        true
    } else {
        false
    }
}

fn parse_expr(source: &str, end: &mut &str, result: &mut Node) -> Error {
    let mut current_token = Token::new();
    let mut token_count = 0;
    let mut err = OK.clone();

    while lex(source, &mut current_token).is_ok() {
        *end = current_token.end;
        let token_length = current_token.end.len();
        if token_length == 0 {
            break;
        }
        if parse_integer(&current_token, result) {
            let lhs_integer = result.clone();
            if lex(current_token.end, &mut current_token).is_err() {
                return err;
            }
            *end = current_token.end;
        } else {
            let mut symbol = Node::new(NodeType::Symbol);
            symbol.symbol = Some(current_token.beginning.to_string());
            *result = symbol.clone();

            if lex(current_token.end, &mut current_token).is_err() {
                return err;
            }
            *end = current_token.end;
            let token_length = current_token.end.len();
            if token_length == 0 {
                break;
            }

            if current_token.beginning == ":" {
                if lex(current_token.end, &mut current_token).is_err() {
                    return err;
                }
                *end = current_token.end;
                let token_length = current_token.end.len();
                if token_length == 0 {
                    break;
                }

                if current_token.beginning == "integer" {
                    let mut var_decl = Node::new(NodeType::VariableDeclaration);
                    let type_node = Node::new(NodeType::Integer);

                    node_add_child(&mut var_decl, type_node);
                    node_add_child(&mut var_decl, symbol);

                    *result = var_decl;
                    return OK.clone();
                }
            }

            println!("Unrecognized token: ");
            print_token(&current_token);

            return err;
        }

        println!("Intermediate node: ");
        print_node(result, 0);
    }

    err
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }

    let path = &args[1];
    match file_contents(path) {
        Ok(contents) => {
            let mut expression = Node::new(NodeType::Program);
            let mut contents_it = contents.as_str();
            let err = parse_expr(contents_it, &mut contents_it, &mut expression);
            print_node(&expression, 0);
            println!();

            print_error(&err);
        }
        Err(err) => {
            eprintln!("Could not open file at {}: {}", path, err);
        }
    }
}
