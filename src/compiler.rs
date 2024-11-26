use crate::{chunk::Chunk, opcode::OpCode, scanner::{Scanner, Token, TokenType}, value::Value};

pub fn compile(source: &str) -> Result<Chunk, ()> {
    let mut parser = Parser::new(source);
    advance(&mut parser);
    consume(&mut parser, TokenType::Eof, "Expect end of expression");
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
}

fn error_at<'src>(parser: &mut Parser<'src>, token: Token<'src>, message: &str) {
    // if things are panicking, don't print anything
    if parser.panic_mode {
        return;
    }
    parser.panic_mode = true;
    eprint!("[line {}] Error", token.line);
    if token.token_type == TokenType::Eof {
        eprint!(" at end");
    } else if token.token_type == TokenType::Error {
        // Nothing.
    } else {
        eprint!(" at '{}'", token.lexeme);
    }
    eprintln!(": {}", message);
    parser.had_error = true;
}

fn error<'src>(parser: &mut Parser<'src>, message: &str) {
    error_at(parser, parser.previous, message);
}

fn error_at_current<'src>(parser: &mut Parser<'src>, message: &str) {
    error_at(parser, parser.current, message);
}

pub fn consume<'src>(parser: &mut Parser<'src>, token_type: TokenType, message: &str) -> bool {
    if parser.current.token_type == token_type {
        advance(parser);
        return true;
    }
    error_at_current(parser, message);
    false
}

fn emit_byte<'src>(parser: &mut Parser<'src>, byte: u8) {
    parser.chunk.write(byte, parser.previous.line);
}

fn emit_return<'src>(parser: &mut Parser<'src>) {
    emit_byte(parser, OpCode::Return as u8);
}

fn emit_bytes<'src>(parser: &mut Parser<'src>, byte1: u8, byte2: u8) {
    emit_byte(parser, byte1);
    emit_byte(parser, byte2);
}

fn emit_constant<'src>(parser: &mut Parser<'src>, value: Value) {
    let const_idx: u8 = make_constant(parser, value);
    emit_bytes(parser, OpCode::Constant as u8, const_idx);
}

fn make_constant<'src>(parser: &mut Parser<'src>, value: Value) -> u8 {
    let idx: usize = parser.chunk.add_const(value);
    if idx > u8::MAX as usize {
        error(parser, "Too many constants in one chunk");
        return 0;
    }
    idx as u8
}

fn expression<'src>(parser: &mut Parser<'src>) {
    parse_precedence(parser, Precedence::Assignment);
}

fn number<'src>(parser: &mut Parser<'src>) {
    let value: f64 = if let Ok(num) = parser.previous.lexeme.parse() {
        num
    } else {
        error_at(parser, parser.previous, "Invalid number");
        return;
    };
    emit_constant(parser, value);
}

fn grouping<'src>(parser: &mut Parser<'src>) {
    expression(parser);
    consume(parser, TokenType::RightParen, "Expect ')' after expression");
}

fn unary<'src>(parser: &mut Parser<'src>) {
    let operator_type: TokenType = parser.previous.token_type;

    // Compile the operand
    parse_precedence(parser, Precedence::Unary);

    // emit the operator instruction
    match operator_type {
        TokenType::Minus => emit_byte(parser, OpCode::Negate as u8),
        _ => unreachable!(),
    }
}

fn binary<'src>(parser: &mut Parser<'src>) {
    // Retrieve the type of the operator from the previous token
    let operator_type: TokenType = parser.previous.token_type;

    // Get the parsing rule associated with the operator type
    let rule: &ParseRule = get_rule(operator_type);

    // Raise the precedence level to parse the right operand and keep left associativity
    parse_precedence(parser, Precedence::from(rule.precedence as u8 + 1));

    match operator_type {
        TokenType::Plus => emit_byte(parser, OpCode::Add as u8),
        TokenType::Minus => emit_byte(parser, OpCode::Subtract as u8),
        TokenType::Star => emit_byte(parser, OpCode::Multiply as u8),
        TokenType::Slash => emit_byte(parser, OpCode::Divide as u8),
        _ => unreachable!(),
    }
}

fn parse_precedence<'src>(parser: &mut Parser<'src>, precedence: Precedence) {
    // Advance to the next token
    advance(parser);
    // Get the ParseRule for the previous token
    let mut rule: &ParseRule = get_rule(parser.previous.token_type);
    // Call the prefix function if it exists
    match rule.prefix {
        Some(prefix) => prefix(parser),
        None => {
            error(parser, "Expect expression");
            return;
        }
    }

    // Continue parsing as long as the current precedence is less than the precedence of the next token
    while precedence <= get_rule(parser.current.token_type).precedence {
        advance(parser);
        rule = get_rule(parser.current.token_type);
        match rule.infix {
            Some(infix) => infix(parser),
            None => return,
        }
    }
}

fn advance<'src>(parser: &mut Parser<'src>) {
    parser.previous = parser.current;

    loop {
        parser.current = parser.scanner.scan_token();
        if parser.current.token_type != TokenType::Error {
            break;
        }
        error_at_current(parser, &parser.current.lexeme);
    }
}

fn end_compiler<'src>(parser: &mut Parser<'src>) {
    emit_return(parser);
    #[cfg(feature = "debug")]
    {
        use crate::debug::disassemble_chunk;
        if !parser.had_error {
            disassemble_chunk(&parser.chunk, parser.chunk.name);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl From<u8> for Precedence {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

type ParseFn = fn(&mut Parser) -> ();

struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

const RULES: [ParseRule; 40] = [
    // TokenType::LeftParen
    ParseRule {
        prefix: Some(grouping),
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::RightParen
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::LeftBrace
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::RightBrace
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Comma
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Dot
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Minus
    ParseRule {
        prefix: Some(unary),
        infix: Some(binary),
        precedence: Precedence::Term,
    },
    // TokenType::Plus
    ParseRule {
        prefix: None,
        infix: Some(binary),
        precedence: Precedence::Term,
    },
    // TokenType::Semicolon
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Slash
    ParseRule {
        prefix: None,
        infix: Some(binary),
        precedence: Precedence::Factor,
    },
    // TokenType::Star
    ParseRule {
        prefix: None,
        infix: Some(binary),
        precedence: Precedence::Factor,
    },
    // TokenType::Bang
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::BangEqual
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Equal
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::EqualEqual
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Greater
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::GreaterEqual
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Less
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::LessEqual
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Identifier
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::String
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Number
    ParseRule {
        prefix: Some(number),
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::And
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Class
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Else
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::False
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::For
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Fun
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::If
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Nil
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Or
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Print
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Return
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Super
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::This
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::True
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Var
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::While
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Error
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // TokenType::Eof
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
];
// Fixed the function to return a reference to a ParseRule without a lifetime parameter
fn get_rule(token_type: TokenType) -> &'static ParseRule {
    &RULES[token_type as usize]
}
