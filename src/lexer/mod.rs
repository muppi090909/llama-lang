use std::mem::transmute;

use cstree::Language;
use logos::Logos;

#[derive(Debug, Clone, Logos)]
#[repr(u16)]
pub enum Token {
    Root,
    #[token(" ")]
    WhiteSpace,
    #[error]
    Error,
    Identifier,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("let")]
    Let,
    #[token("def")]
    Def,
    #[token("extern")]
    Extern,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex(r#"\d+"#)]
    Integer,
    #[regex(r#""[^\s|"]""#)]
    Str,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBrack,
    #[token("]")]
    RBrack,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("->")]
    Arrow,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token("/")]
    Div,
    #[token("*")]
    Mul,
    #[token("=")]
    Assign,
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("<=")]
    LessEq,
    #[token(">=")]
    GreaterEq,
    #[token("==")]
    Equal,
    #[token("!")]
    Not,
    #[token("!=")]
    NotEq,
    #[token("*=")]
    MulEq,
    #[token("/=")]
    DivEq,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Lang;

impl Language for Lang {
    type Kind = Token;

    fn kind_to_raw(kind: Self::Kind) -> cstree::SyntaxKind {
        cstree::SyntaxKind(kind as u16)
    }
    fn kind_from_raw(raw: cstree::SyntaxKind) -> Self::Kind {
        unsafe { transmute::<_, Token>(raw.0) }
    }
}
