use std::char;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::iter::Peekable;
use std::str::{self, Chars};
use std::sync::Arc;

use crate::SyntaxKind;

pub struct Spanned<T: Debug + Clone + PartialEq + Eq> {
    pub token: T,
    pub offset: usize,
}

pub type Token = Arc<TokenData>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenData {
    pub kind: SyntaxKind,
    pub text: String,
}

impl Display for TokenData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.kind, self.text)
    }
}

pub type LexFn = fn(&mut Peekable<Chars>) -> Option<TokenData>;

fn lex_equal(chars: &mut Peekable<Chars>) -> Option<TokenData> {
    chars.next();
    Some(TokenData {
        kind: SyntaxKind::Equal,
        text: "=".to_string(),
    })
}

fn lex_colon(chars: &mut Peekable<Chars>) -> Option<TokenData> {
    chars.next();
    Some(TokenData {
        kind: SyntaxKind::Colon,
        text: ":".to_string(),
    })
}

fn lex_semicolon(chars: &mut Peekable<Chars>) -> Option<TokenData> {
    chars.next();
    Some(TokenData {
        kind: SyntaxKind::Semicolon,
        text: ";".to_string(),
    })
}

fn lex_newline(chars: &mut Peekable<Chars>) -> Option<TokenData> {
    chars.next();
    Some(TokenData {
        kind: SyntaxKind::NewLine,
        text: "\n".to_string(),
    })
}

fn punctuation_tokenizers() -> HashMap<char, LexFn> {
    HashMap::from([
        ('=', lex_equal as LexFn),
        (':', lex_colon),
        (';', lex_semicolon),
        ('\n', lex_newline),
    ])
}

fn lex_whitespace(chars: &mut Peekable<Chars>) -> Option<TokenData> {
    if chars
        .peek()
        .copied()
        .map(|c| c.is_whitespace() && c != '\n')
        != Some(true)
    {
        return None;
    }
    let mut text = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() && c != '\n' {
            text.push(c);
            chars.next();
        } else {
            break;
        }
    }
    Some(TokenData {
        kind: SyntaxKind::Whitespace,
        text,
    })
}

fn lex_ident_or_keyword(chars: &mut Peekable<Chars>) -> Option<TokenData> {
    let mut text = String::new();
    if chars.peek().copied().map(|c| c.is_alphabetic()) != Some(true) {
        return None;
    }
    while let Some(&c) = chars.peek() {
        if c.is_alphanumeric() || c == '_' {
            text.push(c);
            chars.next();
        } else {
            break;
        }
    }
    let kind = match text.as_str() {
        "let" => SyntaxKind::Let,
        "string" => SyntaxKind::Type,
        _ => SyntaxKind::Ident,
    };
    Some(TokenData { kind, text })
}

fn lex_string_literal(chars: &mut Peekable<Chars>) -> Option<TokenData> {
    if chars.peek() != Some(&'"') {
        return None;
    }
    chars.next(); // consume the opening quote
    let mut value = String::new();
    while let Some(&c) = chars.peek() {
        chars.next();
        if c == '"' {
            return Some(TokenData {
                kind: SyntaxKind::StringLiteral,
                text: value,
            });
        }
        value.push(c);
    }
    // Unterminated string literal
    Some(TokenData {
        kind: SyntaxKind::Error,
        text: value,
    })
}

pub fn table_lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();
    let punct = punctuation_tokenizers();

    while let Some(&ch) = chars.peek() {
        if let Some(&lex_fn) = punct.get(&ch) {
            if let Some(tok) = lex_fn(&mut chars) {
                tokens.push(Token::new(tok));
                continue;
            }
        }

        if let Some(tok) = lex_whitespace(&mut chars) {
            tokens.push(Token::new(tok));
            continue;
        }

        if let Some(tok) = lex_ident_or_keyword(&mut chars) {
            tokens.push(Token::new(tok));
            continue;
        }

        if let Some(tok) = lex_string_literal(&mut chars) {
            tokens.push(Token::new(tok));
            continue;
        }

        // fallback: unknown character
        chars.next(); // consume one char
        tokens.push(Token::new(TokenData {
            kind: SyntaxKind::Error,
            text: ch.to_string(),
        }));
    }

    tokens
}
