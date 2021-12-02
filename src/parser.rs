use crate::lexer::KeyWord;
use crate::lexer::Token;
use crate::lexer::TokenResult;

pub trait lexer {
    fn next(&mut self) -> TokenResult;
}

struct Parser<L: lexer> {
    lex: L,
}

impl<L: lexer> Parser<L> {
    fn parse(&mut self) {}

    fn match_global_declaration() {}

    // variable_decl ::= type {'*'} id { ',' {'*'} id } ';'
    fn match_var_define(&mut self) {}

    fn match_type(&mut self) {
        if let Ok(Token::KeyWord(k)) = self.lex.next() {
            
        }
    }
}
