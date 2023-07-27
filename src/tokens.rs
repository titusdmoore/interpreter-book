#[derive(Debug, PartialEq)]
pub enum Tokens {
    LET,
    FN,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,

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
    ASTERISK,
    SLASH,

    // Boolean Tokens
    BANG,
    AND,
    OR,
    EQUALS,
    GREATER,
    LESS,
    GREATEREQUAL,
    LESSEQUAL,
    NOTEQUALS,
}

pub enum TokenMode {
    NORMAL,
    ASSIGN,
}

impl Tokens {
    pub fn to_token(token_str: &str, mode: TokenMode) -> Tokens {
        match token_str {
            "let" => Tokens::LET,
            "fn" => Tokens::FN,
            "true" => Tokens::TRUE,
            "false" => Tokens::FALSE,
            "if" => Tokens::IF,
            "else" => Tokens::ELSE,
            "return" => Tokens::RETURN,

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
            "/" => Tokens::SLASH,
            "*" => Tokens::ASTERISK,

            "!" => Tokens::BANG,
            "&&" => Tokens::AND,
            "||" => Tokens::OR,
            "==" => Tokens::EQUALS,
            ">" => Tokens::GREATER,
            "<" => Tokens::LESS,
            ">=" => Tokens::GREATEREQUAL,
            "<=" => Tokens::LESSEQUAL,
            "!=" => Tokens::NOTEQUALS,

            _ => {
                match mode {
                    TokenMode::NORMAL => {
                        if let Ok(int_val) = &token_str.trim().parse::<i64>() {
                            return Tokens::INTEGER(*int_val);
                        }

                        if Tokens::is_valid_iden(token_str) {
                            return Tokens::IDENTIFIER(token_str.to_string());
                        }
                    }
                    TokenMode::ASSIGN => {
                        if let Ok(int_val) = &token_str.trim().parse::<i64>() {
                            return Tokens::INTEGER(*int_val);
                        }

                        return Tokens::IDENTIFIER(token_str.to_string());
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

    pub fn is_valid_multichar_token_char(ch: char) -> bool {
        ch == '|' || ch == '&' || ch == '=' || ch == '<' || ch == '>' || ch == '!'
    }

    pub fn should_walk_next_char(char_next: char) -> bool {
        Tokens::is_valid_iden_char(char_next) || Tokens::is_valid_multichar_token_char(char_next)
    }
}
