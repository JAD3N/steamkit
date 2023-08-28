mod error;

use std::ops;

pub use error::*;
use indexmap::IndexMap;
use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not},
    character::complete::{alphanumeric1, char, none_of, not_line_ending, one_of, space0, space1},
    combinator::{cut, recognize, opt},
    error::{context, ParseError},
    multi::many1,
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

pub struct KeyValue {
    pub key: String,
    pub value: Value,
    pub macro_: Option<String>,
}

pub enum Value {
    String(String),
    Map(IndexMap<String, KeyValue>),
}

// impl ops::Add for KeyValue {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {

//     }
// }

fn comment(input: &str) -> IResult<&str, &str> {
    recognize(pair(char('/'), not_line_ending))(input)
}

fn unquoted_string(input: &str) -> IResult<&str, &str> {
    recognize(many1(none_of("\"{}\n\r\t ")))(input)
}

fn quoted_string(input: &str) -> IResult<&str, &str> {
    preceded(
        char('"'),
        cut(terminated(
            escaped(none_of("\"\\"), '\\', one_of("\"nt\\")),
            char('"'),
        )),
    )(input)
}

fn string(input: &str) -> IResult<&str, &str> {
    alt((quoted_string, unquoted_string))(input)
}

fn key_value(input: &str) -> IResult<&str, (&str, &str, Option<&str>)> {
    tuple((
        preceded(space0, string),
        preceded(space1, string),
        preceded(space0, opt(comment)),
    ))(input)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::*;

    #[test]
    fn test_me() {
        let mut file = File::open("assets/items_game.txt").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        println!("{:?}", key_value("\"Hello World\" \"Hello World\"     // hello there!").unwrap().1);
    }
}
