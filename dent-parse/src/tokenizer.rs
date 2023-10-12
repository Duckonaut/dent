use crate::{Error, Result};

pub(crate) struct Tokenizer<'s> {
    input: &'s str,
    chars: std::str::Chars<'s>,
    char: Option<char>,
    pos: usize,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Token<'s> {
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Colon,
    String(&'s str),
    Number(&'s str),
    Bool(bool),
    At,
    Comment,
    Eof,
}

impl<'s> Token<'s> {
    pub fn type_name(&self) -> String {
        match self {
            Token::OpenBracket => "BRACKET_OPEN",
            Token::CloseBracket => "BRACKET_CLOSE",
            Token::OpenBrace => "BRACE_OPEN",
            Token::CloseBrace => "BRACE_CLOSE",
            Token::Colon => "COLON",
            Token::String(_) => "STRING",
            Token::Number(_) => "NUMBER",
            Token::Bool(_) => "BOOL",
            Token::Comment => "COMMENT",
            Token::At => "AT",
            Token::Eof => "EOF",
        }
        .to_string()
    }
}

impl<'s> Tokenizer<'s> {
    pub fn new(input: &'s str) -> Tokenizer<'s> {
        let mut chars = input.chars();
        let char = chars.next();
        Tokenizer {
            input,
            chars,
            char,
            pos: 0,
        }
    }

    pub fn next(&mut self) -> Result<Token<'s>> {
        self.skip_whitespace();

        let r = match self.char {
            None => Ok(Token::Eof),
            Some(c) => match c {
                '[' => {
                    self.next_char();
                    Ok(Token::OpenBracket)
                }
                ']' => {
                    self.next_char();
                    Ok(Token::CloseBracket)
                }
                '{' => {
                    self.next_char();
                    Ok(Token::OpenBrace)
                }
                '}' => {
                    self.next_char();
                    Ok(Token::CloseBrace)
                }
                ':' => {
                    self.next_char();
                    Ok(Token::Colon)
                }
                '@' => {
                    self.next_char();
                    Ok(Token::At)
                }
                '#' => {
                    self.next_char();
                    while let Some(c) = self.char {
                        if c == '\n' {
                            break;
                        }
                        self.next_char();
                    }
                    Ok(Token::Comment)
                }
                '"' => {
                    self.next_char();
                    let start = self.pos;
                    while let Some(c) = self.char {
                        if c == '"' {
                            break;
                        }
                        self.next_char();
                    }
                    let end = self.pos;
                    self.next_char();

                    let s = &self.input[start..end];

                    Ok(Token::String(s))
                }
                '0'..='9' => {
                    let start = self.pos;
                    while let Some(c) = self.char {
                        if !c.is_ascii_digit() && c != '.' {
                            break;
                        }
                        self.next_char();
                    }
                    let end = self.pos;
                    let s = &self.input[start..end];
                    Ok(Token::Number(s))
                }
                c if c.is_alphabetic()
                    || c == '_'
                    || c == '-'
                    || c == '+'
                    || c == '.'
                    || c == ','
                    || c == '/'
                    || c == '\\' =>
                {
                    let start = self.pos;
                    while let Some(c) = self.char {
                        if !c.is_alphanumeric() && c != '_' {
                            break;
                        }
                        self.next_char();
                    }
                    let end = self.pos;
                    let s = &self.input[start..end];

                    if s == "true" {
                        return Ok(Token::Bool(true));
                    } else if s == "false" {
                        return Ok(Token::Bool(false));
                    }
                    Ok(Token::String(s))
                }
                _ => Err(Error::UnexpectedChar(c)),
            },
        };
        r
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.char {
            if !c.is_whitespace() {
                break;
            }
            self.next_char();
        }
    }

    fn next_char(&mut self) {
        self.pos += self.char.map(|c| c.len_utf8()).unwrap_or(0);
        self.char = self.chars.next();
    }
}
