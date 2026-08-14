use logos::Logos;

#[derive(Logos)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    #[token("let")] Let,
    #[token("=")] Equal,
    #[token(";")] Semicolon,

    #[regex("[a-zA-Z_]+", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex("(0(x|X)[0-9a-fA-F]+|0(b|B)[01]+|[0-9]+)", |lex| parse_num(lex))]
    Number(u16),
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, ()> {
    let lexer = Token::lexer(source);
    lexer.collect()
}

fn parse_num(lex: &mut logos::Lexer<Token>) -> Option<u16> {
    let mut slice = lex.slice();
    let (base, discard) = match &lex.slice()[0..1] {
        "0x" => (16, true),
        "0b" => (2, true),
        _ => (10, false),
    };
    if discard { slice = &slice[2..] };
    u16::from_str_radix(slice, base).ok()
}
