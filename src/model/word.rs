use crate::model::{random_string, Value};
use crate::provider::Provider;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Strings {}

impl Provider for Strings {
    fn next(&mut self) -> Value {
        Value::String(random_string(None))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmptyString {}

impl Provider for EmptyString {
    fn next(&mut self) -> Value {
        Value::String("".to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WordTitle {}

impl Provider for WordTitle {
    fn next(&mut self) -> Value {
        let s = Strings {}.next().to_string();
        let mut char_vec: Vec<char> = s.chars().collect();
        char_vec[0] = char_vec[0].to_uppercase().nth(0).unwrap();
        Value::String(char_vec.into_iter().collect())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sentence {
    word_count: u32,
}

impl Provider for Sentence {
    fn next(&mut self) -> Value {
        if self.word_count == 0 {
            return Value::String("".to_string());
        }

        let mut segment: Vec<String> = Vec::new();
        for i in 0..self.word_count {
            if i == 0 {
                segment.push(WordTitle {}.next().to_string())
            } else if i == self.word_count - 1 {
                let word_with_dot = format!("{}.", self.next());
                segment.push(word_with_dot)
            } else {
                segment.push(self.next().to_string())
            }
        }

        Value::String(segment.join(" "))
    }
}
