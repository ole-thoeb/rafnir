use std::iter::FromIterator;
use std::marker::PhantomData;
use std::num::{ParseFloatError, ParseIntError};
use std::rc::Rc;
use std::str::FromStr;

use crate::parser::Parser;
use crate::text::location::Location;

#[derive(Clone, Debug)]
pub struct TextState {
    input: Rc<String>,
    location: Location,
}

impl TextState {
    pub fn next(&mut self) -> Option<char> {
        let next = self.peek_internal();
        self.advance_internal(next);
        next.map(|(_, c)| c)
    }

    pub fn peek(&self) -> Option<char> {
        self.peek_internal().map(|(_, c)| c)
    }

    fn peek_internal(&self) -> Option<(usize, char)> {
        let next_str = &self.input[self.location.byte_offset()..];
        next_str.char_indices().next()
    }

    pub fn advance(&mut self) {
        let next = self.peek_internal();
        self.advance_internal(next);
    }

    fn advance_internal(&mut self, next: Option<(usize, char)>) {
        let new_location = match next {
            None => self.location.clone(),
            Some((index, '\n')) => self.location.new_line(index + 1),
            Some((index, _)) => self.location.increment(index + 1)
        };
        self.location = new_location;
    }
}

pub trait TextParser: Parser<State=TextState> {

    fn pars(&self, input: String) -> Result<Self::Value, Self::Error> {
        let state = TextState {
            input: Rc::new(input),
            location : Location::default()
        };
        self.do_pars(state).map(|(_, value)| value)
    }
}

impl<'a, P: Parser<State=TextState>> TextParser for P {}

pub struct Token<E: Clone> {
    token: String,
    error: E,
}

impl<E: Clone> Token<E> {
    pub fn new(token: String, error: E) -> Self {
        Self { token, error }
    }
}

impl<E: Clone> Parser for Token<E> {
    type Value = String;
    type State = TextState;
    type Error = E;

    fn do_pars(&self, mut state: Self::State) -> Result<(Self::State, Self::Value), Self::Error> {
        for expected_char in self.token.chars() {
            match state.next() {
                None => return Err(self.error.clone()),
                Some(found_char) => {
                    if expected_char != found_char {
                        return Err(self.error.clone());
                    }
                }
            }
        }
        Ok((state, self.token.clone()))
    }
}

#[derive(Debug, Clone)]
pub struct Chop<F: Clone, E> {
    f: F,
    _error: PhantomData<E>
}

impl<F, E> Chop<F, E> where F: Fn(char) -> bool + Clone {
    pub fn while_con(predicate: F) -> Self {
        Self { f: predicate, _error: PhantomData::default() }
    }
}

pub fn whitespace<E>() -> Chop<fn(char) -> bool, E> {
    Chop::while_con(char::is_whitespace)
}


impl<F, E> Parser for Chop<F, E> where F: Fn(char) -> bool + Clone {
    type Value = String;
    type State = TextState;
    type Error = E;

    fn do_pars(&self, mut state: Self::State) -> Result<(Self::State, Self::Value), Self::Error> {
        let predicate = &self.f;
        let mut chopped_chars = vec![];
        loop {
            match state.peek() {
                Some(char) if predicate(char) => {
                    chopped_chars.push(char);
                    state.advance();
                }
                _ => return Ok((state, String::from_iter(chopped_chars)))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Number<F, I, R, E: Clone>
    where F: Fn(Result<f64, ParseFloatError>) -> Result<R, E>,
          I: Fn(Result<i64, ParseIntError>) -> Result<R, E>,
{
    float: F,
    integer: I,
    error: E,
}

impl<F, I, R, E: Clone> Number<F, I, R, E>
    where F: Fn(Result<f64, ParseFloatError>) -> Result<R, E>,
          I: Fn(Result<i64, ParseIntError>) -> Result<R, E> {
    pub fn new(float: F, integer: I, error: E) -> Self {
        Self { float, integer, error }
    }
}

impl<F, I, R, E: Clone> Parser for Number<F, I, R, E>
    where F: Fn(Result<f64, ParseFloatError>) -> Result<R, E>,
          I: Fn(Result<i64, ParseIntError>) -> Result<R, E>
{
    type Value = R;
    type State = TextState;
    type Error = E;

    fn do_pars(&self, mut state: Self::State) -> Result<(Self::State, Self::Value), Self::Error> {
        let mut safe_state = state.clone();

        let mut consumed_chars = vec![];
        let mut dot_found = false;
        let mut is_float = false;
        loop {
            match state.next() {
                Some('F') => {
                    is_float = true;
                    break;
                }
                Some('.') if !dot_found => {
                    consumed_chars.push('.');
                    dot_found = true;
                    is_float = true;
                }
                Some(digit) if char::is_digit(digit, 10) => {
                    consumed_chars.push(digit);
                }
                _ => break
            }
        }

        // throw out trailing dot (could be from member calls)
        if let Some('.') = consumed_chars.last() {
            consumed_chars.remove(consumed_chars.len() - 1);
        }

        for _ in 0..consumed_chars.len() {
            safe_state.advance()
        }

        let made_progress = !consumed_chars.is_empty();
        let number_str = String::from_iter(consumed_chars);
        if !made_progress {
            return Err(self.error.clone());
        }

        if is_float {
            let number = f64::from_str(number_str.as_str());
            (&self.float)(number).map(|r| (safe_state, r))
        } else {
            let number = i64::from_str(number_str.as_str());
            (&self.integer)(number).map(|r| (safe_state, r))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::text::text_parser::{Number, TextParser};

    fn str_err<T>(str: &str) -> Result<T, String> {
        Err(String::from(str))
    }

    #[test]
    fn pars_integer() {
        let integer = Number::new(
            |_| Err(String::from("Found float, expected integer")),
            |int_res| int_res.map_err(|e| format!("{}", e)),
            String::from("Expected integer"),
        );
        assert_eq!(4i64, integer.pars(String::from("4")).expect("Correct input"));
        assert_eq!(42424242i64, integer.pars(String::from("42424242")).expect("Correct input"));

        assert_eq!(str_err("Expected integer"), integer.pars(String::from("Abc")));
        assert_eq!(str_err("Found float, expected integer"), integer.pars(String::from("42F")));
        assert_eq!(str_err("Found float, expected integer"), integer.pars(String::from("42.42")));
    }

    #[test]
    fn pars_float() {
        let float = Number::new(
            |int_res| int_res.map_err(|e| format!("{}", e)),
            |_|Err(String::from("Found integer, expected float")),
            String::from("Expected float"),
        );
        assert_eq!(str_err("Found integer, expected float"), float.pars(String::from("4")));
        assert_eq!(str_err("Found integer, expected float"), float.pars(String::from("42424242")));

        assert_eq!(str_err("Expected float"), float.pars(String::from("Abc")));
        assert_eq!(Ok(42f64), float.pars(String::from("42F")));
        assert_eq!(Ok(42.42f64), float.pars(String::from("42.42")));
    }
}