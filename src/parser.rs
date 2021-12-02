use crate::lexer::TokenResult;

pub trait lexer {
    fn lex(&mut self) -> TokenResult;
}

struct Parser<L: lexer> {
    lex: L,
}

impl<L: lexer> Parser<L> {
    fn parse(&mut self) {}

    fn match_global_declaration() {}
}
