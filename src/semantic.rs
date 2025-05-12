#![allow(unused)]

use tower_lsp::lsp_types::{SemanticToken, SemanticTokenType, SemanticTokens, SemanticTokensResult};

use crate::{lex, SyntaxKind};


pub fn semantic_tokens_full(
    text: &str,
) -> Result<Option<SemanticTokensResult>, tower_lsp::jsonrpc::Error> {
    let tokens = lex(text); // Token { kind, text }
    let mut semantic_tokens = vec![];

    let mut char_offset = 0;
    let mut prev_line = 0;
    let mut prev_start_char = 0;

    for token in tokens {
        let token_start = char_offset;
        let token_len = token.text.chars().count();
        char_offset += token_len;

        // Map byte offset to line and character position
        let prefix = &text[..token_start];
        let token_line = prefix.lines().count() - 1;
        let token_col = prefix.lines().last().map_or(0, |l| l.len());

        // Skip unknown tokens
        let kind = match token.kind {
            SyntaxKind::Let => SemanticTokenType::KEYWORD,
            SyntaxKind::Ident => SemanticTokenType::VARIABLE,
            SyntaxKind::Type => SemanticTokenType::TYPE,
            SyntaxKind::StringLiteral => SemanticTokenType::STRING,
            _ => continue,
        };

        let delta_line = token_line - prev_line;
        let delta_start = if delta_line == 0 {
            token_col - prev_start_char
        } else {
            token_col
        };

        semantic_tokens.push(SemanticToken {
            delta_line: delta_line as u32,
            delta_start: delta_start as u32,
            length: token_len as u32,
            token_type: token_type_index(kind),
            token_modifiers_bitset: 0,
        });

        prev_line = token_line;
        prev_start_char = token_col;

        // Advance by 1 for separating tokens
        char_offset += 1;
    }

    Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data: semantic_tokens,
    })))
}

const TOKEN_TYPE_INDEX: &[(&str, u32)] =
    &[("keyword", 0), ("variable", 1), ("type", 2), ("string", 3)];

fn token_type_index(typ: SemanticTokenType) -> u32 {
    TOKEN_TYPE_INDEX
        .iter()
        .find(|(name, _)| *name == typ.as_str())
        .map(|(_, idx)| *idx)
        .unwrap_or(0)
}

pub fn provide_semantic_tokens(source: &str) -> Vec<SemanticToken> {
    let lexed = lex(source);
    let mut char_offset = 0;
    let mut current_line = 0;
    let mut prev_start_char = 0;
    let mut offset_start = 0;
    let mut semantic_tokens = vec![];

    for token in lexed {
        let len = token.text.chars().count();
        char_offset += len;
        if token.kind == SyntaxKind::NewLine {
            current_line += 1;
        }
        // Skip unknown tokens
        let kind = match token.kind {
            SyntaxKind::Let => SemanticTokenType::KEYWORD,
            SyntaxKind::Ident => SemanticTokenType::VARIABLE,
            SyntaxKind::Type => SemanticTokenType::TYPE,
            SyntaxKind::StringLiteral => SemanticTokenType::STRING,
            _ => {
                offset_start += token.text.chars().count();
                continue;
            }
        };

        semantic_tokens.push(SemanticToken {
            delta_line: current_line as u32,
            delta_start: offset_start as u32,
            length: len as u32,
            token_type: token_type_index(kind),
            token_modifiers_bitset: 0,
        });

        offset_start += token.text.chars().count();
    }
    semantic_tokens
}


#[cfg(test)]
mod tests {
    use crate::{Token, TokenData};

    use super::*;

    #[test]
    fn test_name2() {
        let input = "let name: string = \"Abhi\";";
        let lexed = lex(input);
        assert_eq!(
            lexed,
            vec![
                Token::new(TokenData {
                    kind: SyntaxKind::Let,
                    text: "let".to_string()
                }),
                Token::new(TokenData {
                    kind: SyntaxKind::Whitespace,
                    text: " ".to_string()
                }),
                Token::new(TokenData {
                    kind: SyntaxKind::Ident,
                    text: "name".to_string()
                }),
                Token::new(TokenData {
                    kind: SyntaxKind::Colon,
                    text: ":".to_string()
                }),
                Token::new(TokenData {
                    kind: SyntaxKind::Whitespace,
                    text: " ".to_string()
                }),
                Token::new(TokenData {
                    kind: SyntaxKind::Type,
                    text: "string".to_string()
                }),
                Token::new(TokenData {
                    kind: SyntaxKind::Whitespace,
                    text: " ".to_string()
                }),
                Token::new(TokenData {
                    kind: SyntaxKind::Equal,
                    text: "=".to_string()
                }),
                Token::new(TokenData {
                    kind: SyntaxKind::Whitespace,
                    text: " ".to_string()
                }),
                Token::new(TokenData {
                    kind: SyntaxKind::StringLiteral,
                    text: "Abhi".to_string()
                }),
                Token::new(TokenData {
                    kind: SyntaxKind::Semicolon,
                    text: ";".to_string()
                }),
            ]
        );
        let mut char_offset = 0;
        let mut current_line = 0;
        let mut prev_start_char = 0;
        let mut offset_start = 0;
        let mut semantic_tokens = vec![];

        for token in lexed {
            let len = token.text.chars().count();
            char_offset += len;
            if token.kind == SyntaxKind::NewLine {
                current_line += 1;
            }
            // Skip unknown tokens
            let kind = match token.kind {
                SyntaxKind::Let => SemanticTokenType::KEYWORD,
                SyntaxKind::Ident => SemanticTokenType::VARIABLE,
                SyntaxKind::Type => SemanticTokenType::TYPE,
                SyntaxKind::StringLiteral => SemanticTokenType::STRING,
                _ => {
                    offset_start += token.text.chars().count();
                    continue;
                }
            };

            semantic_tokens.push(SemanticToken {
                delta_line: current_line as u32,
                delta_start: offset_start as u32,
                length: len as u32,
                token_type: token_type_index(kind),
                token_modifiers_bitset: 0,
            });

            offset_start += token.text.chars().count();
        }

        let input_sem = semantic_tokens;

        if let Some(first) = input_sem.first() {
            let line = first.delta_line;
            let delta_start = first.delta_start;
            let len = first.length;

            assert_eq!(line, 0);
            assert_eq!(delta_start, 0);
            assert_eq!(len, 3);
        }
        if let Some(first) = input_sem.get(1) {
            let line = first.delta_line;
            let delta_start = first.delta_start;
            let len = first.length;

            assert_eq!(line, 0);
            assert_eq!(delta_start, 4);
            assert_eq!(len, 4);
        }
        if let Some(semantic_token) = input_sem.get(2) {
            let line = semantic_token.delta_line;
            let delta_start = semantic_token.delta_start;
            let len = semantic_token.length;

            assert_eq!(line, 0);
            assert_eq!(delta_start, 10);
            assert_eq!(len, 6);
        }
    }
}
