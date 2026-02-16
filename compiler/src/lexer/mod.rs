// AeroFlow Compiler - Lexer (v1.0 Locked Spec)
// Zero-copy, fast, minimal

use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\r\f]+")] // Skip whitespace
#[logos(skip r"#[^\n]*")]      // Skip # comments
#[logos(skip r"//[^\n]*")]     // Skip // comments
pub enum TokenKind {
    // Keywords - Core
    #[token("let")]
    Let,
    #[token("fn")]
    Fn,
    #[token("pure")]
    Pure,
    #[token("return")]
    Return,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("spawn")]
    Spawn,
    #[token("await")]
    Await,
    #[token("sleep")]
    Sleep,

    // Keywords - Structure
    #[token("actor")]
    Actor,
    #[token("agent")]
    Agent,
    #[token("model")]
    Model,
    #[token("tensor")]
    Tensor,
    #[token("state")]
    State,
    #[token("on")]
    On,

    #[token("timeline")]
    Timeline,
    #[token("distributed")]
    Distributed,
    #[token("at")]
    At,
    #[token("payload")]
    Payload,
    #[token("tick")]
    Tick,
    #[token("and")]
    And,
    #[token("or")]
    Or,

    // Types
    #[token("int")]
    IntType,
    #[token("float")]
    FloatType,
    #[token("string")]
    StringType,
    #[token("bool")]
    BoolType,
    #[token("list")]
    List,
    #[token("dict")]
    Dict,

    // Keywords - IO / Imports / Env
    #[token("render")]
    Render,
    #[token("from")]
    From,
    #[token("env")]
    Env,
    #[token("time")]
    Time,
    #[token("rand")]
    Rand,

    // Identifiers & Literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),
    #[regex("[0-9]+(\\.[0-9]+)?", |lex| lex.slice().parse::<f64>().ok())]
    Number(f64),
    #[regex("\"[^\"]*\"", |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    String(String),

    // Operators & Delimiters
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("->")]
    Arrow,
    #[token("=")]
    Equal,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("<")]
    LAngle,
    #[token(">")]
    RAngle,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    BangEqual,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("?")]
    Question,

    EOF,
}

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: TokenKind::lexer(source),
        }
    }

    pub fn next_token(&mut self) -> TokenKind {
        match self.lexer.next() {
            Some(Ok(token)) => token,
            Some(Err(_)) => {
                let slice = self.lexer.slice();
                panic!("Lexer error at: '{:?}'", slice);
            }
            None => TokenKind::EOF,
        }
    }

    pub fn slice(&self) -> &'a str {
        self.lexer.slice()
    }
}
