use crate::common::Result;
use crate::reader::*;
use std::io::prelude::*;

pub type LexerResult = Result<Lexeme>;

#[derive(Debug, PartialEq, Clone)]
pub enum Lexeme {
    /// EOF
    EOF,

    /// CRLF
    CRLF,

    /// CR, not followed by LF
    CR,

    /// LF, not preceded by CR
    LF,

    /// A sequence of letters (A-Z or a-z)
    Word(String),

    /// A sequence of whitespace (spaces and tabs)
    Whitespace(String),

    /// A punctuation symbol
    Symbol(char),

    /// An integer number
    Number(u32),
}

impl Lexeme {
    pub fn push_to(&self, buf: &mut String) {
        match self {
            Self::Word(s) => buf.push_str(s),
            Self::Whitespace(s) => buf.push_str(s),
            Self::Symbol(c) => buf.push(*c),
            _ => panic!("Cannot format {:?}", self),
        }
    }
}

pub struct Lexer<T> {
    reader: CharOrEofReader<T>,
    _last_pos: RowCol,
}

fn _is_letter(ch: char) -> bool {
    (ch >= 'A' && ch <= 'Z') || (ch >= 'a' && ch <= 'z')
}

fn _is_number(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

fn _is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t'
}

fn _is_symbol(ch: char) -> bool {
    ch == '"'
        || ch == '\''
        || ch == '!'
        || ch == ','
        || ch == '$'
        || ch == '%'
        || ch == '+'
        || ch == '-'
        || ch == '*'
        || ch == '/'
        || ch == '('
        || ch == ')'
        || ch == '='
}

impl<T: BufRead> Lexer<T> {
    pub fn new(reader: T) -> Lexer<T> {
        Lexer {
            reader: CharOrEofReader::new(reader),
            _last_pos: RowCol::new(),
        }
    }

    pub fn read(&mut self) -> LexerResult {
        self._last_pos = self.reader.pos();
        let x = self.reader.read_and_consume()?;
        match x {
            CharOrEof::EOF => Ok(Lexeme::EOF),
            CharOrEof::Char(ch) => {
                if _is_letter(ch) {
                    Ok(Lexeme::Word(self._read_while(ch, _is_letter)?))
                } else if _is_whitespace(ch) {
                    Ok(Lexeme::Whitespace(self._read_while(ch, _is_whitespace)?))
                } else if _is_symbol(ch) {
                    Ok(Lexeme::Symbol(ch))
                } else if ch == '\n' {
                    Ok(Lexeme::LF)
                } else if ch == '\r' {
                    self._read_cr_lf()
                } else {
                    Err(format!("[lexer] Unexpected character {}", ch))
                }
            }
        }
    }

    pub fn last_pos(&self) -> RowCol {
        self._last_pos
    }

    fn _read_while(&mut self, initial: char, predicate: fn(char) -> bool) -> Result<String> {
        let mut result: String = String::new();
        result.push(initial);

        loop {
            let x = self.reader.read()?;
            match x {
                CharOrEof::Char(ch) => {
                    if predicate(ch) {
                        result.push(ch);
                        self.reader.consume()?;
                    } else {
                        break;
                    }
                }
                CharOrEof::EOF => {
                    break;
                }
            }
        }

        Ok(result)
    }

    fn _read_cr_lf(&mut self) -> LexerResult {
        let next = self.reader.read()?;
        if let CharOrEof::Char('\n') = next {
            self.reader.consume()?;
            Ok(Lexeme::CRLF)
        } else {
            Ok(Lexeme::CR)
        }
    }
}

pub struct BufLexer<T> {
    lexer: Lexer<T>,
    _last_read_lexeme: Option<Lexeme>
}

impl<T: BufRead> BufLexer<T> {
    pub fn new(reader: T) -> BufLexer<T> {
        BufLexer {
            lexer: Lexer::new(reader),
            _last_read_lexeme: None,
        }
    }

    /// Reads the next lexeme.
    /// The lexeme is stored and no further reads will be done unless
    /// consume is called.
    pub fn read(&mut self) -> LexerResult {
        match self._last_read_lexeme.clone() {
            Some(x) => Ok(x),
            None => {
                let new_lexeme = self.lexer.read()?;
                self._last_read_lexeme = Some(new_lexeme.clone());
                Ok(new_lexeme)
            }
        }
    }

    /// Consumes the previously read lexeme, allowing further reads.
    pub fn consume(&mut self) {
        self._last_read_lexeme = match self._last_read_lexeme {
            None => panic!("No previously read lexeme!"),
            Some(_) => None,
        }
    }

    /// Tries to read the given word. If the next lexeme is this particular word,
    /// it consumes it and it returns true.
    pub fn try_consume_word(&mut self, word: &str) -> Result<bool> {
        let lexeme = self.read()?;
        match lexeme {
            Lexeme::Word(w) => {
                if w == word {
                    self.consume();
                    Ok(true)
                } else {
                    Ok(false)
                }
            },
            _ => Ok(false)
        }
    }

    pub fn try_consume_symbol(&mut self, ch: char) -> Result<bool> {
        let lexeme = self.read()?;
        match lexeme {
            Lexeme::Symbol(w) => {
                if w == ch {
                    self.consume();
                    Ok(true)
                } else {
                    Ok(false)
                }
            },
            _ => Ok(false)
        }
    }

    pub fn demand_word(&mut self) -> Result<String> {
        let lexeme = self.read()?;
        match lexeme {
            Lexeme::Word(w) => {
                self.consume();
                Ok(w)
            },
            _ => Err(format!("Expected word, found {:?}", lexeme))
        }
    }

    pub fn demand_symbol(&mut self, ch: char) -> Result<()> {
        let lexeme = self.read()?;
        match lexeme {
            Lexeme::Symbol(s) => {
                if s == ch {
                    self.consume();
                    Ok(())
                } else {
                    Err(format!("Expected symbol {}, found {}", ch, s))
                }
            },
            _ => Err(format!("Expected symbol {}, found {:?}", ch, lexeme))
        }
    }

    pub fn demand_eol_or_eof(&mut self) -> Result<()> {
        let lexeme = self.read()?;
        match lexeme {
            Lexeme::CR | Lexeme::CRLF | Lexeme::LF | Lexeme::EOF => {
                self.consume();
                Ok(())
            },
            _ => Err(format!("Expected EOL or EOF, found {:?}", lexeme))
        }
    }

    pub fn skip_whitespace(&mut self) -> Result<()> {
        loop {
            let lexeme = self.read()?;
            match lexeme {
                Lexeme::Whitespace(_) => self.consume(),
                _ => break,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor};

    impl Lexer<BufReader<Cursor<&[u8]>>> {
        pub fn from_bytes(bytes: &[u8]) -> Lexer<BufReader<Cursor<&[u8]>>> {
            let c = Cursor::new(bytes);
            let reader = BufReader::new(c);
            Lexer::new(reader)
        }
    }

    #[test]
    fn test_lexer() {
        let input = b"PRINT \"Hello, world!\"";
        let mut lexer = Lexer::from_bytes(input);
        assert_eq!(lexer.read().unwrap(), Lexeme::Word("PRINT".to_string()));
        assert_eq!(lexer.read().unwrap(), Lexeme::Whitespace(" ".to_string()));
        assert_eq!(lexer.read().unwrap(), Lexeme::Symbol('"'));
        assert_eq!(lexer.read().unwrap(), Lexeme::Word("Hello".to_string()));
        assert_eq!(lexer.read().unwrap(), Lexeme::Symbol(','));
        assert_eq!(lexer.read().unwrap(), Lexeme::Whitespace(" ".to_string()));
        assert_eq!(lexer.read().unwrap(), Lexeme::Word("world".to_string()));
        assert_eq!(lexer.read().unwrap(), Lexeme::Symbol('!'));
        assert_eq!(lexer.read().unwrap(), Lexeme::Symbol('"'));
        assert_eq!(lexer.read().unwrap(), Lexeme::EOF);
    }

    #[test]
    fn test_cr_lf() {
        let input = b"Hi\r\n\n\r";
        let mut lexer = Lexer::from_bytes(input);
        assert_eq!(lexer.read().unwrap(), Lexeme::Word("Hi".to_string()));
        assert_eq!(lexer.read().unwrap(), Lexeme::CRLF);
        assert_eq!(lexer.read().unwrap(), Lexeme::LF);
        assert_eq!(lexer.read().unwrap(), Lexeme::CR);
        assert_eq!(lexer.read().unwrap(), Lexeme::EOF);
    }
}