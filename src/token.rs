use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\r\n\f]+|//[^\n]*")]
pub enum Token {
    // Keywords
    #[token("let")]
    Let,
    #[token("hue")]
    Hue,
    #[token("const")]
    Const,
    #[token("struct")]
    Struct,
    #[token("self")]
    Selfkw,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("break")]
    Break,
    #[token("loop")]
    Loop,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("ret")]
    Ret,
    #[token("unsafe")]
    Unsafe,
    #[token("asm")]
    Asm,
    #[token("extern")]
    Extern,
    #[token("give")]
    Give,
    #[token("pub")]
    Pub,
    #[token("use")]
    Use,
    #[token("mod")]
    Mod,

    // Boolean literals
    #[token("true")]
    True,
    #[token("false")]
    False,

    // Primitive types
    #[token("i8")]
    I8,
    #[token("i16")]
    I16,
    #[token("i32")]
    I32,
    #[token("i64")]
    I64,
    #[token("i128")]
    I128,
    #[token("f32")]
    F32,
    #[token("f64")]
    F64,
    #[token("bool")]
    Bool,
    #[token("void")]
    Void,

    // Multi-character operators must come BEFORE single-character
    // operators that share the same prefix.
    #[token("==")]
    Eq,
    #[token("!=")]
    NotEq,
    #[token("<=")]
    LessEq,
    #[token(">=")]
    GreaterEq,
    #[token("->")]
    Arrow,
    #[token("..")]
    DotDot,
    #[token("::")]
    ColonColon,
    #[token("++")]
    Concat,

    // Single-character operators
    #[token("=")]
    Assign,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("!")]
    Bang,
    #[token(".")]
    Dot,

    // Punctuation
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("@")]
    At,

    // Delimiters
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

    // Numeric literals: Float must come before Int so that
    // "3.14" is matched as a float, not as "3" "." "14".
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().unwrap())]
    FloatLit(f64),
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    IntLit(i64),

    // String literals
    #[regex(r#""[^"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    StringLit(String),
    // Identifiers must be LAST so they do not steal exact matches
    // from keywords and primitive types.
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    // Mode directives
    #[regex(r"!\#[a-zA-Z_-]+", |lex| lex.slice().to_string())]
    ModeDirective(String),

    // End of file marker
    EOF,
}
