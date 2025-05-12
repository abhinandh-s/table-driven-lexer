
use crate::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxNodeData, Token};

pub fn parse_tokens_to_cst(tokens: &[Token]) -> SyntaxNode {
    let mut i = 0;
    let mut decls = Vec::new();

    while i < tokens.len() {
        if tokens.get(i).map(|t| &t.kind) != Some(&SyntaxKind::Let) {
            break;
        }

        let mut children = Vec::new();

        children.push(SyntaxElement::Token(tokens[i].clone())); // let
        i += 1;

        if let Some(tok) = tokens.get(i) {
            if tok.kind == SyntaxKind::Ident {
                children.push(SyntaxElement::Token(tok.clone()));
                i += 1;
            }
        }

        if let Some(tok) = tokens.get(i) {
            if tok.kind == SyntaxKind::Colon {
                children.push(SyntaxElement::Token(tok.clone()));
                i += 1;
            }
        }

        if let Some(tok) = tokens.get(i) {
            if tok.kind == SyntaxKind::Type {
                children.push(SyntaxElement::Token(tok.clone()));
                i += 1;
            }
        }

        if let Some(tok) = tokens.get(i) {
            if tok.kind == SyntaxKind::Equal {
                children.push(SyntaxElement::Token(tok.clone()));
                i += 1;
            }
        }

        if let Some(tok) = tokens.get(i) {
            if tok.kind == SyntaxKind::StringLiteral {
                children.push(SyntaxElement::Token(tok.clone()));
                i += 1;
            }
        }

        if let Some(tok) = tokens.get(i) {
            if tok.kind == SyntaxKind::Semicolon {
                children.push(SyntaxElement::Token(tok.clone()));
                i += 1;
            }
        }

        decls.push(SyntaxElement::Node(
            SyntaxNodeData {
                kind: SyntaxKind::VarDecl,
                children,
            }
            .into(),
        ));
    }

    SyntaxNodeData::new(SyntaxKind::Root, decls).into()
}

#[derive(Debug)]
pub struct VarDecl {
    pub name: String,
    pub ty: String,
    pub value: String,
}

pub fn lower_to_ast(root: &SyntaxNode) -> Vec<VarDecl> {
    let mut decls = Vec::new();
    for node in root.child_nodes() {
        if node.kind() != SyntaxKind::VarDecl {
            continue;
        }

        let tokens = node.tokens();
        let name = tokens
            .iter()
            .find(|t| t.kind == SyntaxKind::Ident)
            .unwrap()
            .text
            .clone();
        let ty = tokens
            .iter()
            .find(|t| t.kind == SyntaxKind::Type)
            .unwrap()
            .text
            .clone();
        let value = tokens
            .iter()
            .find(|t| t.kind == SyntaxKind::StringLiteral)
            .unwrap()
            .text
            .clone();

        decls.push(VarDecl { name, ty, value });
    }

    decls
}

pub fn analyze(decls: &[VarDecl]) {
    for decl in decls {
        if decl.ty != "string" {
            println!("Error: Unsupported type '{}'", decl.ty);
        }
        if decl.value.is_empty() {
            println!("Warning: Empty string for '{}'", decl.name);
        }
    }
}

pub fn compile(decls: &[VarDecl]) -> String {
    let mut out = String::from("{\n");
    for d in decls {
        out.push_str(&format!("  \"{}\": \"{}\",\n", d.name, d.value));
    }
    out.push('}');
    out
}

