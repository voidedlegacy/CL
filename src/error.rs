#[derive(Debug, PartialEq)]
enum ErrorType {
    ErrorNone = 0,
    ErrorArguments,
    ErrorType,
    ErrorGeneric,
    ErrorSyntax,
    ErrorTodo,
    ErrorMax,
}

#[derive(Debug)]
struct Error {
    error_type: ErrorType,
    msg: Option<String>,
}

impl Error {
    fn new(error_type: ErrorType, msg: Option<String>) -> Self {
        Error { error_type, msg }
    }
}

fn print_error(err: &Error) {
    if err.error_type == ErrorType::ErrorNone {
        return;
    }
    print!("ERROR: ");
    assert!(ErrorType::ErrorMax as i32 == 6);
    match err.error_type {
        ErrorType::ErrorTodo => println!("TODO (not implemented)"),
        ErrorType::ErrorSyntax => println!("Invalid syntax"),
        ErrorType::ErrorType => println!("Mismatched types"),
        ErrorType::ErrorArguments => println!("Invalid arguments"),
        ErrorType::ErrorGeneric => {},
        _ => println!("Unknown error type..."),
    }
    if let Some(ref msg) = err.msg {
        println!("     : {}", msg);
    }
}

fn main() {
    let ok = Error::new(ErrorType::ErrorNone, None);
    let syntax_error = Error::new(ErrorType::ErrorSyntax, Some("Unexpected token".to_string()));
    
    print_error(&ok);
    print_error(&syntax_error);
}

