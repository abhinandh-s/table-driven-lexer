use std::sync::Arc;

use crate::{SyntaxKind, Token};


#[derive(Debug, Clone)]
pub enum SyntaxElement {
    Token(Token),
    Node(SyntaxNode),
}

pub type SyntaxNode = Arc<SyntaxNodeData>;

#[derive(Debug, Clone)]
pub struct SyntaxNodeData {
    pub kind: SyntaxKind,
    pub children: Vec<SyntaxElement>,
}

impl SyntaxNodeData {
    pub fn new(kind: SyntaxKind, children: Vec<SyntaxElement>) -> Self {
        SyntaxNodeData { kind, children }
    }

    pub fn tokens(&self) -> Vec<&Token> {
        self.children
            .iter()
            .filter_map(|el| match el {
                SyntaxElement::Token(tok) => Some(tok),
                _ => None,
            })
            .collect()
    }

    pub fn child_nodes(&self) -> Vec<&SyntaxNode> {
        self.children
            .iter()
            .filter_map(|el| match el {
                SyntaxElement::Node(n) => Some(n),
                _ => None,
            })
            .collect()
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }
}

