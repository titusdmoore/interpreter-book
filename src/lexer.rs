use crate::tokens::TokenMode;
use crate::tokens::Tokens;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
        };

        lexer.read_char();
        return lexer;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            return;
        }

        self.position = self.read_position;
        self.read_position += 1;

        if self.input.as_bytes()[self.position].is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn read_identifier(&mut self) -> &str {
        let internal_pos = self.position;

        while Tokens::is_valid_iden_char(self.input.as_bytes()[self.read_position] as char) {
            self.read_char();
        }

        &self.input[internal_pos..self.read_position]
    }

    pub fn next_token(&mut self, mode: TokenMode) -> Tokens {
        let mut raw_input_slice = &self.input[self.position..self.read_position];

        if Tokens::is_valid_iden_char(
            raw_input_slice
                .chars()
                .next()
                .expect("Something went wrong with the single &str to char for next_token"),
        ) {
            raw_input_slice = self.read_identifier();
        }

        let token: Tokens = Tokens::to_token(raw_input_slice, mode);
        self.read_char();
        return token;
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::TokenMode;
    use crate::tokens::Tokens;

    #[test]
    fn test_lexer() {
        let input = String::from(
            "
            let myInt = 5;
            let myInt2 = 10;

            let add = fn(a, b) {
                a + b; 
            };
            let result = add(myInt, myInt2);
",
        );
        let expected_result: Vec<Tokens> = vec![
            Tokens::LET,
            Tokens::IDENTIFIER(String::from("myInt")),
            Tokens::ASSIGN,
            Tokens::INTEGER(5),
            Tokens::SEMICOLON,
            Tokens::LET,
            Tokens::IDENTIFIER(String::from("myInt2")),
            Tokens::ASSIGN,
            Tokens::INTEGER(10),
            Tokens::SEMICOLON,
            Tokens::LET,
            Tokens::IDENTIFIER(String::from("add")),
            Tokens::ASSIGN,
            Tokens::FN,
            Tokens::OPENPAREN,
            Tokens::IDENTIFIER(String::from("a")),
            Tokens::COMMA,
            Tokens::IDENTIFIER(String::from("b")),
            Tokens::CLOSEPAREN,
            Tokens::OPENCURLY,
            Tokens::IDENTIFIER(String::from("a")),
            Tokens::PLUS,
            Tokens::IDENTIFIER(String::from("b")),
            Tokens::SEMICOLON,
            Tokens::CLOSECURLY,
            Tokens::SEMICOLON,
            Tokens::LET,
            Tokens::IDENTIFIER(String::from("result")),
            Tokens::ASSIGN,
            Tokens::IDENTIFIER(String::from("add")),
            Tokens::OPENPAREN,
            Tokens::IDENTIFIER(String::from("myInt")),
            Tokens::COMMA,
            Tokens::IDENTIFIER(String::from("myInt2")),
            Tokens::CLOSEPAREN,
            Tokens::SEMICOLON,
        ];

        let mut lexer = super::Lexer::new(input);
        let mut res: Vec<Tokens> = Vec::new();

        for _ in 0..expected_result.len() {
            let mut mode = TokenMode::NORMAL;
            if res.len() > 0 {
                mode = match res[std::cmp::max(res.len() - 1, 0)] {
                    Tokens::ASSIGN => TokenMode::ASSIGN,
                    _ => TokenMode::NORMAL,
                };
            }

            res.push(lexer.next_token(mode));
        }

        assert_eq!(expected_result, res);
    }
}
