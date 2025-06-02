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


/*********************************************************/

#[derive(Debug)]
struct TrieNode {
    kind: Option<SyntaxKind>,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode { kind: None, children: HashMap::new() }
    }

    fn insert(&mut self, sequence: &str, kind: SyntaxKind) {
        let mut node = self;
        for ch in sequence.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
        }
        node.kind = Some(kind);
    }
}


fn build_operator_trie() -> TrieNode {
    let mut root = TrieNode::new();
    root.insert("=", SyntaxKind::Equal);
    root.insert("==", SyntaxKind::EqualEqual);
    root.insert("=>", SyntaxKind::FatArrow);
    root.insert("=<", SyntaxKind::EqualLess);
    root.insert(":=", SyntaxKind::ColonEqual);
    root.insert(":", SyntaxKind::Colon);
    root.insert("::", SyntaxKind::DoubleColon);
    root.insert(";", SyntaxKind::Semicolon);
    root.insert("\n", SyntaxKind::NewLine);
    // Add more as needed
    root
}

/// # Example
/// ```
/// let operator_trie = build_operator_trie();
/// 
/// while let Some(&ch) = chars.peek() {
///     if let Some(tok) = lex_operator(&mut chars, &operator_trie) {
///         tokens.push(Token::new(tok));
///         continue;
///     }
/// 
///     // fallback for identifier, number, etc.
/// }
/// ``` 
fn lex_operator(chars: &mut Peekable<Chars>, trie: &TrieNode) -> Option<TokenData> {
    let mut node = trie;
    let mut matched = None;
    let mut matched_text = String::new();
    let mut temp_buffer = String::new();

    let mut iter = chars.clone();

    while let Some(&ch) = iter.peek() {
        if let Some(next_node) = node.children.get(&ch) {
            temp_buffer.push(ch);
            iter.next();
            node = next_node;
            if let Some(kind) = node.kind {
                matched = Some((kind, temp_buffer.clone()));
                matched_text = temp_buffer.clone();
            }
        } else {
            break;
        }
    }

    // Actually consume the characters now
    for _ in 0..matched_text.len() {
        chars.next();
    }

    matched.map(|(kind, text)| TokenData { kind, text })
}


fn take_while<F: Fn(char) -> bool>(chars: &mut Peekable<Chars>, pred: F) -> String {
    let mut result = String::new();
    while let Some(&c) = chars.peek() {
        if pred(c) {
            chars.next();
            result.push(c);
        } else {
            break;
        }
    }
    result
}

fn lex_whitespace(chars: &mut Peekable<Chars>) -> Option<TokenData> {
    let text = take_while(chars, |c| c.is_whitespace() && c != '\n');
    if text.is_empty() {
        None
    } else {
        Some(TokenData {
            kind: SyntaxKind::Whitespace,
            text,
        })
    }
}