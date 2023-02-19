use std::str::Chars;

use unicode_xid::UnicodeXID;

use crate::{error::LangError, span::Span, ErrorSink};

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum TokenKind {
    Comment,
    Whitespace,
    Newline,

    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Colon,
    Equals,
    Comma,
    Pipe,

    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Rem,
    And,
    Or,
    Not,

    Ident,
    LetKw,
    FnKw,
    IfKw,
    ElseKw,
    ForKw,
    InKw,
    WhileKw,
    StringTy,
    IntTy,
    FloatTy,
    BoolTy,
    SizeTy,
    DurationTy,
    DateTy,
    TimeTy,
    DateTimeTy,

    String,
    Int,
    Float,
    Bool,
    Size,
    Duration,

    Unknown,
}

pub fn lex(source: &str) -> (Vec<Token>, ErrorSink) {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    while let Some((token, error)) = lexer.consume_token() {
        tokens.push(token);
        if let Some(error) = error {
            lexer.errors.push(error);
        }
    }
    (tokens, lexer.errors)
}

struct Lexer<'c> {
    chars: Chars<'c>,
    start: u32,
    current: u32,
    errors: ErrorSink,
}

impl<'c> Lexer<'c> {
    fn new(source: &'c str) -> Self {
        Self { chars: source.chars(), start: 0, current: 0, errors: ErrorSink::new() }
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn first(&mut self) -> char {
        self.chars.clone().next().unwrap_or('\0')
    }

    fn second(&mut self) -> char {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next().unwrap_or('\0')
    }

    fn finish_token(&mut self) -> Span {
        let res = Span { start: self.start, end: self.current };
        self.start = self.current;
        res
    }

    fn bump(&mut self) -> Option<char> {
        self.current += 1;
        self.chars.next()
    }

    fn bump_with(&mut self, kind: TokenKind) -> TokenKind {
        self.bump();
        kind
    }

    fn eat_while(&mut self, predicate: impl FnMut(char) -> bool) {
        self.eat_while_and(predicate, |_| ());
    }

    fn eat_while_and(&mut self, mut predicate: impl FnMut(char) -> bool, mut f: impl FnMut(char)) {
        while predicate(self.first()) && !self.is_eof() {
            f(self.first());
            self.bump();
        }
    }

    fn ident(&mut self, first: char) -> TokenKind {
        let mut word = String::from(first);
        self.eat_while_and(UnicodeXID::is_xid_continue, |c| word.push(c));
        match word.as_str() {
            "true" | "false" => TokenKind::Bool,
            "let" => TokenKind::LetKw,
            "fn" => TokenKind::FnKw,
            "if" => TokenKind::IfKw,
            "else" => TokenKind::ElseKw,
            "for" => TokenKind::ForKw,
            "in" => TokenKind::InKw,
            "while" => TokenKind::WhileKw,
            "String" => TokenKind::StringTy,
            "Int" => TokenKind::IntTy,
            "Float" => TokenKind::FloatTy,
            "Bool" => TokenKind::BoolTy,
            "Size" => TokenKind::SizeTy,
            "Duration" => TokenKind::DurationTy,
            "Date" => TokenKind::DateTy,
            "Time" => TokenKind::TimeTy,
            "DateTime" => TokenKind::DateTimeTy,
            _ => TokenKind::Ident,
        }
    }

    fn string(&mut self) -> Option<LangError> {
        while let Some(c) = self.bump() {
            match c {
                '"' => return None,
                '\\' if self.first() == '\\' || self.first() == '"' => {
                    self.bump();
                }
                _ => (),
            }
        }
        let span = Span { start: self.start, end: self.start + 1 };
        Some(LangError::UnterminatedString { span: span.into() })
    }

    fn numeric(&mut self) -> (TokenKind, Option<LangError>) {
        let mut kind = TokenKind::Int;
        self.eat_while(|c| matches!(c, '0'..='9' | '_'));
        if self.first() == '.' {
            self.bump();
            if self.second().is_ascii_digit() {
                kind = TokenKind::Float;
                self.bump();
                self.eat_while(|c| matches!(c, '0'..='9' | '_'));
            } else {
                let span = self.finish_token();
                let error = Some(LangError::FloatWithoutFractional { span: span.into() });
                return (TokenKind::Float, error);
            }
        }

        if self.first().is_alphabetic() {
            let mut suffix = String::new();
            self.eat_while_and(char::is_alphabetic, |c| suffix.push(c));
            match suffix.as_str() {
                "b" | "kb" | "mb" | "gb" | "kib" | "mib" | "gib" => return (TokenKind::Size, None),
                "day" | "hr" | "min" | "s" | "ms" | "us" | "ns" => {
                    return (TokenKind::Duration, None)
                }
                _ => {
                    #[expect(clippy::cast_possible_truncation)]
                    let start = self.current - suffix.len() as u32;
                    let span = Span { start, end: self.current };
                    let error = Some(LangError::UnknownNumericSuffix { suffix, span: span.into() });
                    return (kind, error);
                }
            }
        }

        if self.first() == '-' {}

        (kind, None)
    }

    fn consume_token(&mut self) -> Option<(Token, Option<LangError>)> {
        let first = self.bump()?;
        let mut error = None;
        let kind = match first {
            '\n' => TokenKind::Newline,
            '/' if self.first() == '/' => {
                self.eat_while(|c| c != '\n');
                self.finish_token();
                TokenKind::Comment
            }
            c if c.is_whitespace() => {
                self.eat_while(|c| c.is_whitespace() && c != '\n');
                self.finish_token();
                TokenKind::Whitespace
            }

            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '=' => TokenKind::Equals,
            '|' if self.first() != '|' => TokenKind::Pipe,

            '+' => TokenKind::Add,
            '-' => TokenKind::Sub,
            '*' => TokenKind::Mul,
            '/' => TokenKind::Div,
            '^' => TokenKind::Pow,
            '%' => TokenKind::Rem,
            '&' if self.first() == '&' => self.bump_with(TokenKind::And),
            // there's no need to check for `|` here because it's already
            // checked when piping
            '|' => self.bump_with(TokenKind::Or),
            '!' => TokenKind::Not,

            '"' => {
                error = self.string();
                TokenKind::String
            }
            '0'..='9' => {
                let (kind, maybe_error) = self.numeric();
                error = maybe_error;
                kind
            }

            c if UnicodeXID::is_xid_start(c) || c == '_' => self.ident(c),

            _ => TokenKind::Unknown,
        };
        let span = self.finish_token();
        Some((Token { kind, span }, error))
    }
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use super::{lex, Token};

    #[expect(clippy::needless_pass_by_value)]
    fn check(source: &str, expected: Expect) {
        let mut actual = String::new();
        for Token { kind, span } in lex(source).0 {
            actual += &format!("{kind:?} {span:?}\n");
        }
        expected.assert_eq(&actual);
    }

    #[test]
    fn simple() {
        check(
            "()[]:=,|+-*/^%&||!",
            expect![[r#"
            OpenParen 0..1
            CloseParen 1..2
            OpenBracket 2..3
            CloseBracket 3..4
            Colon 4..5
            Equals 5..6
            Comma 6..7
            Pipe 7..8
            Add 8..9
            Sub 9..10
            Mul 10..11
            Div 11..12
            Pow 12..13
            Rem 13..14
            Unknown 14..15
            Or 15..17
            Not 17..18
        "#]],
        );
    }

    #[test]
    fn keywords() {
        check(
            "let fn if else for in while String Int Float Bool Size Duration Date Time DateTime",
            expect![[r#"
                LetKw 0..3
                FnKw 4..6
                IfKw 7..9
                ElseKw 10..14
                ForKw 15..18
                InKw 19..21
                WhileKw 22..27
                StringTy 28..34
                IntTy 35..38
                FloatTy 39..44
                BoolTy 45..49
                SizeTy 50..54
                DurationTy 55..63
                DateTy 64..68
                TimeTy 69..73
                DateTimeTy 74..82
            "#]],
        );
    }

    #[test]
    fn idents() {
        check(
            "abc d le letf",
            expect![[r#"
                Ident 0..3
                Ident 4..5
                Ident 6..8
                Ident 9..13
            "#]],
        );
    }
}
