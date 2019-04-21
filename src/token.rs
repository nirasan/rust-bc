pub enum Token {
    Number(float64),

    Plus,
    Minus,
    Asterisk,
    Slash,

    SemiColon,
    Eof,
}