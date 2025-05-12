#![allow(clippy::unwrap_used)]

fn main() {
    let input = "let x: string = \"hello\";";
    table_driven_lexer::table_lex(input).iter().for_each(|tok| {
        println!("{}", tok);
    });
}

#[cfg(test)]
mod qtests {
    use quickcheck::quickcheck;
    use table_driven_lexer::*;

    quickcheck! {
        fn parsing_does_not_panic(input: String) -> bool {
            let tokens = lex(&input);
            let cst = parse_tokens_to_cst(&tokens);
            let _ast = lower_to_ast(&cst);
            true // if we reached here, no panic = pass
        }

        fn compile_outputs_valid_json(input: String) -> bool {
            let tokens = lex(&input);
            let cst = parse_tokens_to_cst(&tokens);
            let ast = lower_to_ast(&cst);
            let json = compile(&ast);
            serde_json::from_str::<serde_json::Value>(&json).is_ok()
        }
    }
}



