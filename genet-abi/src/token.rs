use env;
use std::fmt;

/// A token value.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Token(u32);

impl Token {
    /// Returns a null token.
    pub fn null() -> Token {
        Token(0)
    }

    /// Returns the corresponded string.
    pub fn to_string(self) -> String {
        env::string(self)
    }
}

impl Into<u32> for Token {
    fn into(self) -> u32 {
        self.0
    }
}

impl<'a> From<&'a Token> for Token {
    fn from(id: &'a Token) -> Token {
        *id
    }
}

impl From<u32> for Token {
    fn from(id: u32) -> Token {
        Token(id)
    }
}

impl From<String> for Token {
    fn from(id: String) -> Token {
        Token::from(id.as_str())
    }
}

impl<'a> From<&'a String> for Token {
    fn from(id: &'a String) -> Token {
        Token::from(id.as_str())
    }
}

impl<'a> From<&'a str> for Token {
    fn from(id: &'a str) -> Token {
        env::token(id)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&Token::to_string(*self))
    }
}
