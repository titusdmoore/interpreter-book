#[derive(Debug, PartialEq)]
pub enum Tokens {
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

pub enum TokenMode {
    NORMAL,
    ASSIGN,
}

impl Tokens {
    pub fn to_token(token_str: &str, mode: TokenMode) -> Tokens {
        println!("token_str: {}", token_str);
        // if let Ok(int_val) = &token_str.parse::<i64>() {
        //     return Tokens::INTEGER(*int_val);
        // }

        match token_str {
            "let" => Tokens::LET,
            "fn" => Tokens::FN,
            "=" => Tokens::ASSIGN,
            "0" => Tokens::EOF,

            "{" => Tokens::OPENCURLY,
            "}" => Tokens::CLOSECURLY,
            "(" => Tokens::OPENPAREN,
            ")" => Tokens::CLOSEPAREN,
            "" => Tokens::OPENBRACKET,
            "]" => Tokens::CLOSEBRACKET,

            "," => Tokens::COMMA,
            ";" => Tokens::SEMICOLON,

            "+" => Tokens::PLUS,
            "-" => Tokens::MINUS,

            _ => {
                match mode {
                    TokenMode::NORMAL => {
                        if Tokens::is_valid_iden(token_str) {
                            return Tokens::IDENTIFIER(token_str.to_string());
                        }
                    }
                    TokenMode::ASSIGN => {
                        if let Ok(int_val) = &token_str.parse::<i64>() {
                            return Tokens::INTEGER(*int_val);
                        }

                        return Tokens::INVALID;
                    }
                }

                Tokens::INVALID
            }
        }
    }

    pub fn is_valid_iden_char(ch: char) -> bool {
        ch.is_ascii_alphanumeric() || ch == '_'
    }

    pub fn is_valid_iden(iden: &str) -> bool {
        iden.chars().all(|ch| Tokens::is_valid_iden_char(ch))
    }
}
