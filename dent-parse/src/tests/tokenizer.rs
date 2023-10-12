use super::*;

#[test]
fn number() {
    let mut tokenizer = Tokenizer::new("123");
    assert_eq!(tokenizer.next(), Ok(Token::Number("123")));
    assert_eq!(tokenizer.next(), Ok(Token::Eof));
}

#[test]
fn numbers() {
    let mut tokenizer = Tokenizer::new("123 1 2 3 1.0 2.0 11.2 11.");
    assert_eq!(tokenizer.next(), Ok(Token::Number("123")));
    assert_eq!(tokenizer.next(), Ok(Token::Number("1")));
    assert_eq!(tokenizer.next(), Ok(Token::Number("2")));
    assert_eq!(tokenizer.next(), Ok(Token::Number("3")));
    assert_eq!(tokenizer.next(), Ok(Token::Number("1.0")));
    assert_eq!(tokenizer.next(), Ok(Token::Number("2.0")));
    assert_eq!(tokenizer.next(), Ok(Token::Number("11.2")));
    assert_eq!(tokenizer.next(), Ok(Token::Number("11.")));
    assert_eq!(tokenizer.next(), Ok(Token::Eof));
}

#[test]
fn string() {
    let mut tokenizer = Tokenizer::new("hello");
    assert_eq!(tokenizer.next(), Ok(Token::String("hello")));
    assert_eq!(tokenizer.next(), Ok(Token::Eof));
}

#[test]
fn strings() {
    let mut tokenizer = Tokenizer::new("hello \"dear\" world");
    assert_eq!(tokenizer.next(), Ok(Token::String("hello")));
    assert_eq!(tokenizer.next(), Ok(Token::String("dear")));
    assert_eq!(tokenizer.next(), Ok(Token::String("world")));
    assert_eq!(tokenizer.next(), Ok(Token::Eof));
}

#[test]
fn bool() {
    let mut tokenizer = Tokenizer::new("true false");
    assert_eq!(tokenizer.next(), Ok(Token::Bool(true)));
    assert_eq!(tokenizer.next(), Ok(Token::Bool(false)));
    assert_eq!(tokenizer.next(), Ok(Token::Eof));
}

#[test]
fn special_characters() {
    let mut tokenizer = Tokenizer::new("[]{}@:");
    assert_eq!(tokenizer.next(), Ok(Token::OpenBracket));
    assert_eq!(tokenizer.next(), Ok(Token::CloseBracket));
    assert_eq!(tokenizer.next(), Ok(Token::OpenBrace));
    assert_eq!(tokenizer.next(), Ok(Token::CloseBrace));
    assert_eq!(tokenizer.next(), Ok(Token::At));
    assert_eq!(tokenizer.next(), Ok(Token::Colon));
    assert_eq!(tokenizer.next(), Ok(Token::Eof));
}

#[test]
fn comments() {
    let mut tokenizer = Tokenizer::new("hello # world\n");
    assert_eq!(tokenizer.next(), Ok(Token::String("hello")));
    assert_eq!(tokenizer.next(), Ok(Token::Comment));
    assert_eq!(tokenizer.next(), Ok(Token::Eof));
}

#[test]
fn comments2() {
    let mut tokenizer = Tokenizer::new("hello # world\n# comment");
    assert_eq!(tokenizer.next(), Ok(Token::String("hello")));
    assert_eq!(tokenizer.next(), Ok(Token::Comment));
    assert_eq!(tokenizer.next(), Ok(Token::Comment));
    assert_eq!(tokenizer.next(), Ok(Token::Eof));
}

#[test]
fn list() {
    let mut tokenizer = Tokenizer::new("[1 2 3] [ 1 2 a ]");

    assert_eq!(tokenizer.next(), Ok(Token::OpenBracket));
    assert_eq!(tokenizer.next(), Ok(Token::Number("1")));
    assert_eq!(tokenizer.next(), Ok(Token::Number("2")));
    assert_eq!(tokenizer.next(), Ok(Token::Number("3")));
    assert_eq!(tokenizer.next(), Ok(Token::CloseBracket));
    assert_eq!(tokenizer.next(), Ok(Token::OpenBracket));
    assert_eq!(tokenizer.next(), Ok(Token::Number("1")));
    assert_eq!(tokenizer.next(), Ok(Token::Number("2")));
    assert_eq!(tokenizer.next(), Ok(Token::String("a")));
    assert_eq!(tokenizer.next(), Ok(Token::CloseBracket));
    assert_eq!(tokenizer.next(), Ok(Token::Eof));
}

#[test]
fn dict() {
    let mut tokenizer = Tokenizer::new("{a: 1 b: 2}");
    assert_eq!(tokenizer.next(), Ok(Token::OpenBrace));
    assert_eq!(tokenizer.next(), Ok(Token::String("a")));
    assert_eq!(tokenizer.next(), Ok(Token::Colon));
    assert_eq!(tokenizer.next(), Ok(Token::Number("1")));
    assert_eq!(tokenizer.next(), Ok(Token::String("b")));
    assert_eq!(tokenizer.next(), Ok(Token::Colon));
    assert_eq!(tokenizer.next(), Ok(Token::Number("2")));
    assert_eq!(tokenizer.next(), Ok(Token::CloseBrace));
    assert_eq!(tokenizer.next(), Ok(Token::Eof));
}
