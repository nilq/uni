#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Block(Vec<Token>),
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BoolLiteral,
    Symbol,
    Operator,
    Identifier,
    Keyword,
    Whitespace,
    EOF,
}

#[derive(Debug, Copy, Clone)]
pub struct TokenPosition {
    pub line: usize,
    pub col:  usize,
}

impl Default for TokenPosition {
    fn default() -> Self {
        TokenPosition {
            line: 1,
            col:  0,
        }
    }
}

impl TokenPosition {
    pub fn new(line: usize, col: usize) -> TokenPosition {
        TokenPosition {
            line: line,
            col: col,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pos:            TokenPosition,
    content:        String,
}

#[allow(dead_code)]
impl Token {
    pub fn new(token_type: TokenType, pos: TokenPosition, content: String) -> Token {
        Token {
            token_type: token_type,
            pos:        pos,
            content:    content,
        }
    }

    pub fn pos(&self) -> &TokenPosition {
        &self.pos
    }

    pub fn content(&self) -> &String {
        &self.content
    }
}

impl<'a> PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.token_type == other.token_type
    }

    fn ne(&self, other: &Token) -> bool {
        self.token_type != other.token_type
    }
}