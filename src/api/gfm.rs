pub trait GfmChar {
    fn is_ascii_punctuation_character(&self) -> bool;
}

impl GfmChar for char {
    fn is_ascii_punctuation_character(&self) -> bool {
        std::matches!(
            self,
            '!' | '"'
                | '#'
                | '$'
                | '%'
                | '&'
                | '\''
                | '('
                | ')'
                | '*'
                | '+'
                | ','
                | '-'
                | '.'
                | '/'
                | ':'
                | ';'
                | '<'
                | '='
                | '>'
                | '?'
                | '@'
                | '['
                | '\\'
                | ']'
                | '^'
                | '_'
                | '`'
                | '{'
                | '|'
                | '}'
                | '~'
        )
    }
}

