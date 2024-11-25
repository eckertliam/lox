use crate::{chunk::Chunk, opcode::OpCode, scanner::{Scanner, Token, TokenType}};

pub fn compile(source: &str) -> Result<Chunk, ()> {
    let mut parser = Parser::new(source);
    parser.advance();
    parser.consume(TokenType::Eof, "Expect end of expression");
    if parser.had_error {
        Err(())
    } else {
        Ok(parser.chunk)
    }
}

struct Parser<'src> {
    scanner: Scanner<'src>,
    chunk: Chunk,
    current: Token<'src>,
    previous: Token<'src>,
    had_error: bool,
    panic_mode: bool,
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { 
            scanner: Scanner::new(source), 
            chunk: Chunk::new("main"), 
            current: Token::default(), 
            previous: Token::default(),
            had_error: false,
            panic_mode: false,
        }
    }

    fn error_at(&mut self, token: Token<'src>, message: &str) {
        // if things are panicking, don't print anything
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        eprint!("[line {}] Error", token.line);
        if token.token_type == TokenType::Eof {
            eprint!(" at end");
        } else if token.token_type == TokenType::Error {
            // Nothing.
        } else {
            eprint!(" at '{}'", token.lexeme);
        }
        eprintln!(": {}", message);
        self.had_error = true;
    }

    fn error(&mut self, message: &str) {
        self.error_at(self.previous, message);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current, message);
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> bool {
        if self.current.token_type == token_type {
            self.advance();
            return true;
        }
        self.error_at_current(message);
        false
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write(byte, self.previous.line);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return as u8);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.scanner.scan_token();
            if self.current.token_type != TokenType::Error {
                break;
            }
            self.error_at_current(&self.current.lexeme);
        }
    }
}
