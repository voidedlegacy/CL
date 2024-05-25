
pub const WHITESPACE: &str = " \r\n";
pub const DELIMITERS: &str = " \r\n,():";

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub beginning: usize,
    pub end: usize,
}

impl Token {
    pub fn new(beginning: usize, end: usize) -> Self {
        Token { beginning, end }
    }
}

pub fn print_token(token: &Token, source: &str) {
    println!("{}", &source[token.beginning..token.end]);
}

pub fn lex(source: &str, token: &mut Token) -> Result<(), String> {
    if source.is_empty() || token.beginning >= source.len() {
        return Err("Cannot lex empty source.".to_string());
    }

    token.beginning += source[token.beginning..]
        .chars()
        .take_while(|c| WHITESPACE.contains(*c))
        .count();
    token.end = token.beginning;
    if token.end >= source.len() {
        return Ok(());
    }

    token.end += source[token.beginning..]
        .chars()
        .take_while(|c| !DELIMITERS.contains(*c))
        .count();
    if token.end == token.beginning {
        token.end += 1;
    }
    Ok(())
}

pub fn token_string_equalp(string: &str, token: &Token, source: &str) -> bool {
    &source[token.beginning..token.end] == string
}
