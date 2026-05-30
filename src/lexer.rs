use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("return")]
    Return,

    #[regex("[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    IntLit(i64),

    #[token(";")]
    Semi,

    Error,
}