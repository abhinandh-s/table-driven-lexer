use crate::{SyntaxKind, Token, TokenData};

pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            c if c.is_whitespace() => {
                tokens.push(Token::new(TokenData {
                    kind: SyntaxKind::Whitespace,
                    text: " ".into(),
                }));
                chars.next();
            }
            ':' => {
                tokens.push(Token::new(TokenData {
                    kind: SyntaxKind::Colon,
                    text: ":".into(),
                }));
                chars.next();
            }
            '\n' => {
                tokens.push(Token::new(TokenData {
                    kind: SyntaxKind::NewLine,
                    text: "\n".into(),
                }));
                chars.next();
            }
            '=' => {
                tokens.push(Token::new(TokenData {
                    kind: SyntaxKind::Equal,
                    text: "=".into(),
                }));
                chars.next();
            }
            ';' => {
                tokens.push(Token::new(TokenData {
                    kind: SyntaxKind::Semicolon,
                    text: ";".into(),
                }));
                chars.next();
            }
            '"' => {
                chars.next();
                let mut value = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next();
                        break;
                    }
                    value.push(c);
                    chars.next();
                }
                tokens.push(Token::new(TokenData {
                    kind: SyntaxKind::StringLiteral,
                    text: value,
                }));
            }
            c if c.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let kind = match ident.as_str() {
                    "let" => SyntaxKind::Let,
                    "string" => SyntaxKind::Type,
                    _ => SyntaxKind::Ident,
                };
                tokens.push(Token::new(TokenData { kind, text: ident }));
            }
            _ => {
                tokens.push(Token::new(TokenData {
                    kind: SyntaxKind::Error,
                    text: ch.to_string(),
                }));
                chars.next();
            }
        }
    }

    tokens
}


