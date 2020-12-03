use std::iter::Peekable;
use std::str::CharIndices;

use phf::phf_map;
use unicode_xid::UnicodeXID;

use crate::error::LexicalError;
use crate::token::Token;
use crate::token::Token::{CharLiteral, Lifetime};

#[allow(unused)]
pub struct Lexer<'input> {
    text: &'input str,
    chars: Peekable<CharIndices<'input>>,
    lookahead: Option<(usize, char)>,
    last_tokens: [Option<Token<'input>>; 2],
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "spec"  => Token::Spec,
    "options"  => Token::Options,
    "typo"  => Token::Typo,
    "default"  => Token::Default,
    "rule"  => Token::Tokenizer,
    "node"  => Token::Node,
    "ast"  => Token::Ast,
    "template"  => Token::Template,
    "impl"  => Token::Impl,
    "define"  => Token::Define,
};

fn is_identifier_start(c: char) -> bool {
    UnicodeXID::is_xid_start(c) || c == '_'
}

fn is_identifier_continue(c: char) -> bool {
    UnicodeXID::is_xid_continue(c) || c == '_'
}

pub type OriginSpanned<T> = (usize, T, usize);

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            text: input,
            chars: input.char_indices().peekable(),
            lookahead: None,
            last_tokens: [None, None],
        }
    }

    fn take_until<F>(&mut self, mut terminate: F) -> Option<usize>
    where
        F: FnMut(char) -> bool,
    {
        loop {
            match self.lookahead {
                None => {
                    return None;
                }
                Some((idx1, c)) => {
                    if terminate(c) {
                        return Some(idx1);
                    } else {
                        self.bump();
                    }
                }
            }
        }
    }

    fn take_while<F>(&mut self, mut keep_going: F) -> Option<usize>
    where
        F: FnMut(char) -> bool,
    {
        self.take_until(|c| !keep_going(c))
    }

    fn word(&mut self, idx0: usize) -> (usize, &'input str, usize) {
        match self.take_while(is_identifier_continue) {
            Some(end) => (idx0, &self.text[idx0..end], end),
            None => (idx0, &self.text[idx0..], self.text.len()),
        }
    }

    fn string_or_char_literal(
        &mut self,
        idx0: usize,
        quote: char,
        variant: fn(&'input str) -> Token<'input>,
    ) -> Option<OriginSpanned<Token<'input>>> {
        let mut escape = false;
        let terminate = |c: char| {
            if escape {
                escape = false;
                false
            } else if c == '\\' {
                escape = true;
                false
            } else {
                c == quote
            }
        };
        match self.take_until(terminate) {
            Some(idx1) => {
                self.bump(); // consume the closing quote
                let text = &self.text[idx0 + 1..idx1]; // do not include quotes in the str
                Some((idx0, variant(text), idx1 + 1))
            }
            None => None,
        }
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        self.lookahead = self.chars.next();
        self.lookahead
    }

    // Saw a `'`, could either be: `'a` or `'a'`.
    fn lifetimeish(&mut self, idx0: usize) -> Result<(usize, Token<'input>, usize), LexicalError> {
        match self.lookahead {
            Some((_, c)) => {
                if is_identifier_start(c) {
                    let (start, word, end) = self.word(idx0);
                    match self.lookahead {
                        Some((idx2, '\'')) => {
                            self.bump();
                            let text = &self.text[idx0 + 1..idx2];
                            Ok((idx0, CharLiteral(text), idx2 + 1))
                        }
                        _ => Ok((start, Lifetime(word), end)),
                    }
                } else {
                    match self.string_or_char_literal(idx0, '\'', CharLiteral) {
                        Some(x) => Ok(x),
                        None => Err(LexicalError::UnrecognisedToken(idx0, idx0, "".to_string())),
                    }
                }
            }
            _ => Err(LexicalError::UnrecognisedToken(idx0, idx0, "".to_string())),
        }
    }

    fn next(&mut self) -> Option<Result<(usize, Token<'input>, usize), LexicalError>> {
        loop {
            match self.chars.next() {
                Some((start, ch)) if ch == '_' || UnicodeXID::is_xid_start(ch) => {
                    let end;

                    loop {
                        if let Some((i, ch)) = self.chars.peek() {
                            if !UnicodeXID::is_xid_continue(*ch) {
                                end = *i;
                                break;
                            }
                            self.bump();
                        } else {
                            end = self.text.len();
                            break;
                        }
                    }

                    let id = &self.text[start..end];

                    return if let Some(w) = KEYWORDS.get(id) {
                        Some(Ok((start, *w, end)))
                    } else {
                        Some(Ok((start, Token::Identify(id), end)))
                    };
                }
                Some((_, ch)) if ch.is_whitespace() => (),
                Some((i, ';')) => return Some(Ok((i, Token::Semicolon, i + 1))),
                Some((i, '$')) => return Some(Ok((i, Token::Dollar, i + 1))),
                Some((idx0, ':')) => {
                    return Some(Ok((idx0, Token::Colon, idx0 + 1)));
                }
                Some((idx0, '"')) => {
                    self.bump();
                    return Some(
                        match self.string_or_char_literal(idx0, '"', Token::StringLiteral) {
                            Some(x) => Ok(x),
                            None => {
                                Err(LexicalError::UnrecognisedToken(idx0, idx0, "".to_string()))
                            }
                        },
                    );
                }
                Some((i, '{')) => return Some(Ok((i, Token::OpenCurlyBrace, i + 1))),
                Some((i, '}')) => return Some(Ok((i, Token::CloseCurlyBrace, i + 1))),
                Some((idx0, '\'')) => return Some(self.lifetimeish(idx0)),
                Some((i, '/')) => match self.bump() {
                    Some((_, '/')) => {
                        self.take_until(|c| c == '\n');
                        continue;
                    }
                    _ => return Some(Ok((i, Token::Divide, i + 1))),
                },
                Some((i, '-')) => {
                    return match self.chars.peek() {
                        Some((_, '>')) => {
                            self.bump();
                            Some(Ok((i, Token::ArrowAssign, i + 2)))
                        }
                        _ => Some(Ok((i, Token::Subtract, i + 1))),
                    };
                }
                Some((start, _)) => {
                    let mut end;

                    let until_nl = self.take_until(|c| c == '\n');
                    match until_nl {
                        None => {}
                        Some(end) => {
                            let string = self.text[start..end].to_owned();
                            let has_end = string.contains(';');
                            if has_end {
                                return Some(Ok((
                                    start,
                                    Token::Pattern(&self.text[start..end - 1]),
                                    end - start,
                                )));
                            }
                        }
                    }

                    loop {
                        if let Some((i, ch)) = self.bump() {
                            end = i;

                            if ch.is_whitespace() {
                                break;
                            }
                        } else {
                            end = self.text.len();
                            break;
                        }
                    }

                    return Some(Err(LexicalError::UnrecognisedToken(
                        start,
                        end,
                        self.text[start..end].to_owned(),
                    )));
                }
                None => return None,
            }
        }
    }
}

pub type Spanned<Token, Location, Error> = Result<(Location, Token, Location), Error>;

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<'input>, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next();

        self.last_tokens = [
            self.last_tokens[1],
            match token {
                Some(Ok((_, n, _))) => Some(n),
                _ => None,
            },
        ];

        token
    }
}
