use crate::{error::*, model::*, vectors::*};
use std::path::Path;

/*
pub fn load(input: &str) {
    let mut tokenizer = Tokenizer::new(input);
}
*/

struct Json {}

#[derive(PartialEq, Debug)]
enum Token {
    OpenCurly,
    ClosedCurly,
    Identifier(String),
    Colon,
    End,
}

struct Tokenizer {
    pub data: Vec<char>,
    // pub data: String,
    pub index: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            data: input.chars().collect(),
            index: 0,
        }
    }

    // returns the new char
    pub fn advance(&mut self) -> Option<char> {
        if self.index >= self.data.len() {
            return None;
        }

        let c: char = self.data[self.index];
        self.index = self.index + 1;
        return Some(c);
    }

    pub fn move_until(&mut self, is_finished: impl Fn(char) -> bool) {
        loop {
            match self.advance() {
                Some(v) => {
                    if is_finished(v) {
                        self.index = self.index - 1;
                        return;
                    }
                }
                None => return,
            }
        }
    }

    pub fn move_to_num(&mut self) {
        self.move_until(|c| c.is_numeric());
    }

    pub fn move_to_char(&mut self, ct: char) {
        self.move_until(|c| c == ct);
    }

    pub fn move_to_line_end(&mut self) {
        self.move_until(|c| c == '\n');
    }

    pub fn get_current(&self) -> Option<char> {
        if self.index >= self.data.len() {
            return None;
        } else {
            return Some(self.data[self.index]);
        }
    }

    pub fn extract(&self, start: usize, end: usize) -> Option<String> {
        if start == end {
            return None;
        }
        if start > end {
            return None;
        }
        if start > self.data.len() || end > self.data.len() {
            return None;
        }

        let mut ret = String::new();
        let sub = &self.data[start..end];
        for c in sub {
            ret.push(*c);
        }
        Some(ret)
    }

    pub fn get_next_token(&mut self) -> Result<Token, Error> {
        // move until we find a character we recognize
        loop {
            let c: char = match self.get_current() {
                Some(v) => v,
                None => return Ok(Token::End),
            };

            if c == '{' {
                // open curly

                self.advance();
                return Ok(Token::OpenCurly);

                //
            } else if c == '}' {
                // closed curly

                self.advance();
                return Ok(Token::ClosedCurly);

                //
            } else if c == '\"' {
                // identifier

                self.advance();

                let start = self.index;
                self.move_to_char('"');
                let end = self.index;

                self.advance();

                let sub = match self.extract(start, end) {
                    Some(v) => v,
                    None => return Ok(Token::End),
                };

                return Ok(Token::Identifier(sub));

                //
            } else {
                //unknown character. Continue past it.
                self.advance();
            }
        }
    }

    pub fn peek_token(&mut self) -> Result<Token, Error> {
        let orig = self.index;
        let ret = self.get_next_token();
        self.index = orig;
        return ret;
    }

    pub fn starts_with(&self, input: &str) -> bool {
        let mut i: usize = 0;
        for c in input.chars() {
            if c != self.data[i + self.index] {
                return false;
            }
            i = i + 1;
        }
        return true;
    }
}

mod test {
    use super::*;

    #[test]
    fn tokens_curlies() {
        let input = "{}";
        let mut tokenizer = Tokenizer::new(&input);

        assert_eq!(tokenizer.get_next_token().unwrap(), Token::OpenCurly);
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::ClosedCurly);
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::End);
    }

    #[test]
    fn tokens_identifier() {
        let input = "{ \"first_idea\" }";
        let mut tokenizer = Tokenizer::new(&input);

        assert_eq!(tokenizer.get_next_token().unwrap(), Token::OpenCurly);
        assert_eq!(
            tokenizer.get_next_token().unwrap(),
            Token::Identifier("first_idea".into())
        );
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::ClosedCurly);
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::End);
    }
}
