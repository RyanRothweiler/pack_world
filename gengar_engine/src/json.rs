use crate::{error::*, model::*, vectors::*};
use std::{
    collections::{HashMap, VecDeque},
    path::Path,
};

pub fn load_file(file_path: &Path) -> Result<JsonNode, Error> {
    let file_data = std::fs::read_to_string(file_path)?;
    load(&file_data)
}

pub fn load(input: &str) -> Result<JsonNode, Error> {
    let mut tokenizer = Tokenizer::new(input);

    tokenizer.get_next_token()?.require(Token::OpenCurly)?;

    return load_block(&mut tokenizer);
}

fn load_block(tokenizer: &mut Tokenizer) -> Result<JsonNode, Error> {
    let mut head = JsonNode::new();

    loop {
        let token = tokenizer.get_next_token()?;
        match token {
            // start of a new entry
            Token::String(entry_id) => {
                tokenizer.get_next_token()?.require(Token::Colon)?;

                let data = get_data(tokenizer)?;
                head.entries.insert(entry_id, data);
            }

            Token::End | Token::ClosedCurly => return Ok(head),
            _ => continue,
        }
    }
}

fn get_data(tokenizer: &mut Tokenizer) -> Result<JsonData, Error> {
    let data_token = tokenizer.get_next_token()?;
    match data_token {
        // String
        Token::String(data) => {
            return Ok(JsonData::String(data));
        }

        // Float
        Token::Float(data) => {
            return Ok(JsonData::Float(data));
        }

        // Array
        Token::OpenBracket => {
            let mut data: Vec<JsonData> = vec![];
            loop {
                let pk = tokenizer.peek_token()?;
                match pk {
                    // Array ends when we hit a closed bracket
                    Token::ClosedBracket => break,
                    _ => data.push(get_data(tokenizer)?),
                }
            }

            return Ok(JsonData::Array(data));
        }

        // Nested data
        Token::OpenCurly => {
            let data = load_block(tokenizer)?;
            return Ok(JsonData::Class(data));
        }

        _ => return Err(Error::JsonInvalidToken),
    };
}

#[derive(PartialEq, Debug, Clone)]
pub enum JsonData {
    String(String),
    Float(f64),
    Array(Vec<JsonData>),
    Class(JsonNode),
}

#[derive(PartialEq, Debug, Clone)]
pub struct JsonNode {
    pub entries: HashMap<String, JsonData>,
}

impl JsonNode {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn get(&self, path: Vec<String>) -> Option<JsonData> {
        return JsonNode::get_queue(self, VecDeque::from(path));
    }

    pub fn get_float(&self, path: Vec<String>) -> Option<f64> {
        match JsonNode::get_queue(self, VecDeque::from(path)) {
            Some(d) => match d {
                JsonData::Float(v) => return Some(v),
                _ => return None,
            },
            None => return None,
        }
    }

    pub fn get_class(&self, path: Vec<String>) -> Option<JsonNode> {
        match JsonNode::get_queue(self, VecDeque::from(path)) {
            Some(d) => match d {
                JsonData::Class(v) => return Some(v),
                _ => return None,
            },
            None => return None,
        }
    }

    fn get_queue(node: &JsonNode, path: VecDeque<String>) -> Option<JsonData> {
        match path.len() {
            // empty path string. Path is invalid.
            0 => return None,

            // We're at the end of the path
            1 => {
                match node.entries.get(&path[0]) {
                    Some(v) => return Some(v.clone()),
                    None => return None,
                };
            }

            // multiple paths. use the top one
            _ => {
                match node.entries.get(&path[0]) {
                    Some(data) => match data {
                        JsonData::Class(node) => {
                            let mut p = path.clone();
                            p.pop_front();
                            return JsonNode::get_queue(node, p);
                        }
                        _ => return None,
                    },
                    None => return None,
                };
            }
        }
    }
}

#[derive(PartialEq, Debug)]
enum Token {
    OpenCurly,
    ClosedCurly,
    OpenBracket,
    ClosedBracket,
    String(String),
    Float(f64),
    Colon,
    End,
}

impl Token {
    pub fn require(&self, t: Token) -> Result<(), Error> {
        if *self == t {
            return Ok(());
        }
        return Err(Error::JsonInvalidToken);
    }
}

struct Tokenizer {
    pub data: Vec<char>,
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
            } else if c == ']' {
                // open bracked

                self.advance();
                return Ok(Token::ClosedBracket);

                //
            } else if c == '[' {
                // open bracked

                self.advance();
                return Ok(Token::OpenBracket);

                //
            } else if c == ':' {
                // colon

                self.advance();
                return Ok(Token::Colon);

                //
            } else if c.is_numeric() || c == '-' {
                // float

                let mut neg = 1.0;
                if c == '-' {
                    self.advance();
                    neg = -1.0;
                }

                // found number
                self.move_to_num();
                let num_start = self.index;

                self.move_until(|c| !c.is_numeric() && c != '.');

                let num_end = self.index;

                // Tokenizer didn't move, so at end of string
                if num_start == num_end {
                    return Ok(Token::End);
                }

                let sub = match self.extract(num_start, num_end) {
                    Some(v) => v,
                    None => return Ok(Token::End),
                };
                let num: f64 = sub.parse()?;
                return Ok(Token::Float(num * neg));

                //
            } else if c == '"' {
                // string

                self.advance();

                let start = self.index;
                self.move_to_char('"');
                let end = self.index;

                self.advance();

                let sub = match self.extract(start, end) {
                    Some(v) => v,
                    None => return Ok(Token::End),
                };

                return Ok(Token::String(sub));

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
    fn tokens() {
        let input = "{ \"first_idea\" : 123 }";
        let mut tokenizer = Tokenizer::new(&input);

        assert_eq!(tokenizer.get_next_token().unwrap(), Token::OpenCurly);

        assert_eq!(
            tokenizer.get_next_token().unwrap(),
            Token::String("first_idea".into())
        );
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Colon);
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::Float(123.0));

        assert_eq!(tokenizer.get_next_token().unwrap(), Token::ClosedCurly);
        assert_eq!(tokenizer.get_next_token().unwrap(), Token::End);
    }

    #[test]
    fn single_string() {
        let input = "{ \"first_idea\" : \"string here man\", \"second_idea\": \"even more\" }";
        let data = load(&input).unwrap();

        assert_eq!(data.entries.keys().len(), 2);

        assert_eq!(
            data.entries["first_idea"],
            JsonData::String("string here man".into())
        );
        assert_eq!(
            data.entries["second_idea"],
            JsonData::String("even more".into())
        );
    }

    #[test]
    fn single_float() {
        let input = "{ \"first_idea\" : 10.0, \"second_idea\": 10.5 }";
        let data = load(&input).unwrap();

        assert_eq!(data.entries.keys().len(), 2);

        assert_eq!(
            data.get(vec!["first_idea".into()]),
            Some(JsonData::Float(10.0))
        );
        assert_eq!(
            data.get(vec!["second_idea".into()]),
            Some(JsonData::Float(10.5))
        );
    }

    #[test]
    fn nested() {
        let input = "{ \"first_idea\" : { \"second_idea\": 10.5 }, \"hey man\": \"whats up\" }";
        let data = load(&input).unwrap();

        assert_eq!(data.entries.keys().len(), 2);

        assert_eq!(
            data.get(vec!["first_idea".into(), "second_idea".into()]),
            Some(JsonData::Float(10.5))
        );
        assert_eq!(
            data.get(vec!["hey man".into()]),
            Some(JsonData::String("whats up".into()))
        );
    }

    #[test]
    fn array_strings() {
        let input =
            "{ \"first_idea\" : [ \"string one\", \"string two\" ], \"hey man\": \"whats up\" }";
        let data = load(&input).unwrap();

        assert_eq!(data.entries.keys().len(), 2);

        assert_eq!(
            data.get(vec!["hey man".into()]),
            Some(JsonData::String("whats up".into()))
        );

        let ray: Vec<JsonData> = vec![
            JsonData::String("string one".into()),
            JsonData::String("string two".into()),
        ];

        assert_eq!(
            data.get(vec!["first_idea".into()]),
            Some(JsonData::Array(ray))
        );
    }
}
