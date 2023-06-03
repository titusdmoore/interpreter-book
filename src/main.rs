fn main() {}

enum Tokens {
    LET,
    FN,
    ASSIGN,
    IDENTIFIER(String),
    INVALID,
    EOF,

    INTEGER(i64),
    // Commented out until we get to how to store string values without stepping on ident token
    // STRING(String),
    OPENCURLY,
    CLOSECURLY,
    OPENPAREN,
    CLOSEPAREN,
    OPENBRACKET,
    CLOSEBRACKET,

    COMMA,
    SEMICOLON,

    PLUS,
    MINUS,
}

impl Tokens {
    fn to_token(token_str: String) -> Tokens {
        if let Ok(int_val) = &token_str.parse::<i64>() {
            return Tokens::INTEGER(*int_val);
        }

        match token_str.as_str() {
            "let" => Tokens::LET,
            "fn" => Tokens::FN,
            "=" => Tokens::ASSIGN,
            "" => Tokens::EOF,

            "{" => Tokens::OPENCURLY,
            "}" => Tokens::CLOSECURLY,
            "(" => Tokens::OPENPAREN,
            ")" => Tokens::CLOSEPAREN,
            "[" => Tokens::OPENBRACKET,
            "]" => Tokens::CLOSEBRACKET,

            "," => Tokens::COMMA,
            ";" => Tokens::SEMICOLON,

            "+" => Tokens::PLUS,
            "-" => Tokens::MINUS,

            _ => {
                if token_str
                    .as_bytes()
                    .into_iter()
                    .all(|b| b.is_ascii_alphanumeric())
                {
                    return Tokens::IDENTIFIER(token_str);
                }

                Tokens::INVALID
            }
        }
    }
}
