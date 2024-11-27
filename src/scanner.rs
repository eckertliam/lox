pub struct Scanner<'src> {
    source: &'src str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'src> Scanner<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { source, start: 0, current: 0, line: 1 }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&self, token_type: TokenType) -> Token<'src> {
        Token { token_type, lexeme: &self.source[self.start..self.current], line: self.line }
    }

    fn error_token(&self, message: &'static str) -> Token<'src> {
        Token { token_type: TokenType::Error, lexeme: message, line: self.line }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                    break;
                }
                _ => break,
            }
        }
    }


    fn string(&mut self) -> Token<'src> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        self.advance();
        self.make_token(TokenType::String)
    }

    fn number(&mut self) -> Token<'src> {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token<'src> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        self.identifier_token()
    }

    fn identifier_token(&mut self) -> Token<'src> {
        let lexeme = &self.source[self.start..self.current];
        let token_type = match lexeme {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        Token { token_type, lexeme, line: self.line }
    }

    pub fn scan_token(&mut self) -> Token<'src> {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => if self.match_next('/') {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
                return self.scan_token();
            } else {
                self.make_token(TokenType::Slash)
            },
            '*' => self.make_token(TokenType::Star),
            '!' => if self.match_next('=') {
                self.make_token(TokenType::BangEqual)
            } else {
                self.make_token(TokenType::Bang)
            },
            '=' => if self.match_next('=') {
                self.make_token(TokenType::EqualEqual)
            } else {
                self.make_token(TokenType::Equal)
            },
            '<' => if self.match_next('=') {
                self.make_token(TokenType::LessEqual)
            } else {
                self.make_token(TokenType::Less)
            },
            '>' => if self.match_next('=') {
                self.make_token(TokenType::GreaterEqual)
            } else {
                self.make_token(TokenType::Greater)
            },
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier_token(),
            _ => self.error_token("Unexpected character."),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    // Special
    Error,
    Eof,
    // Empty token
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'src> {
    pub token_type: TokenType,
    pub lexeme: &'src str,
    pub line: usize,
}

impl<'src> Default for Token<'src> {
    fn default() -> Self {
        Self { token_type: TokenType::Empty, lexeme: "", line: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_token {
        ($tokens:expr, $index:expr, $token_type:expr) => {
            assert_eq!($tokens[$index].token_type, $token_type);
        };
    }

    #[test]
    fn test_binary_operations() {
        let src = "1.567 * 20";
        let mut scanner = Scanner::new(src);
        let mut tokens = Vec::new();
        while !scanner.is_at_end() {
            tokens.push(scanner.scan_token());
        }
        assert_eq!(tokens.len(), 3);
        assert_token!(tokens, 0, TokenType::Number);
        assert_token!(tokens, 1, TokenType::Star);
        assert_token!(tokens, 2, TokenType::Number);
    }
}
