pub enum Token {
    Number(f64),

    Plus,
    Minus,
    Asterisk,
    Slash,

    SemiColon,
    Eof,
}