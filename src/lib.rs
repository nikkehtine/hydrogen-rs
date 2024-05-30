#[derive(Debug, PartialEq)]
pub enum TokenType {
    Exit,
    IntLit,
    Semi,
}

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub value: Option<String>,
}
