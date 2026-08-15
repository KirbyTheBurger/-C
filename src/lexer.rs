use logos::Logos;

use crate::error::Error;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    #[token("print")] Print,

    #[regex("[a-zA-Z_]+", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex("(0(x|X)[0-9a-fA-F]+|0(b|B)[01]+|[0-9]+)", |lex| parse_num(lex))]
    Number(u16),
}

#[derive(Debug, PartialEq)]
pub struct  SpannedToken {
    pub token: Token,
    pub span: std::ops::Range<usize>,
}

pub fn tokenize(source: &str) -> Result<Vec<SpannedToken>, Vec<Error>> {
    let mut tokens = vec![];
    let mut errors = vec![];

    for (result, span) in Token::lexer(source).spanned() {
        match result {
            Ok(token) => tokens.push(SpannedToken { token, span }),
            Err(_) => errors.push(Error::new("Unexpected character(s)", span)),
        }
    }

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}

fn parse_num(lex: &mut logos::Lexer<Token>) -> Option<u16> {
    let mut slice = lex.slice();
    let (base, discard) = if slice.len() <= 2 {
        (10, false)
    } else {
        match &lex.slice()[0..2] {
            "0x" => (16, true),
            "0b" => (2, true),
            _ => (10, false),
        }
    };
    if discard { slice = &slice[2..] };
    u16::from_str_radix(slice, base).ok()
}
