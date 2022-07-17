use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Parser;
impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    fn parse_line(&self, line: String) -> Item {
        let re = Regex::new(r"(.*)at(.*)").unwrap();
        let groups = re
            .captures(&line)
            .expect(format!("Could not parse the line {}", line).as_str());

        let name = groups
            .get(1)
            .expect(format!("Could not parse name for line {}", line).as_str())
            .as_str()
            .trim();
        let price = groups
            .get(2)
            .expect(format!("Could not parse price for line {}", line).as_str())
            .as_str()
            .trim()
            .parse::<f64>()
            .expect(format!("Could not parse price for line {}", line).as_str());

        Item::new(name.to_owned(), price)
    }

    pub fn parse(&self, file: &File) -> Vec<Item> {
        let file_buffer = BufReader::new(file);

        file_buffer
            .lines()
            .map(|line_res| line_res.expect("Error reading line"))
            .map(|line| self.parse_line(line))
            .collect()
    }
}

#[derive(PartialEq)]
pub struct Item {
    pub name: String,
    pub price: f64,
}

impl Item {
    pub fn new(name: String, price: f64) -> Self {
        Item {
            name: name,
            price: price,
        }
    }
}

#[test]
fn happy_case() {
    let p = Parser::new();
    let expected = Item::new("1 belegtes Brot mit Schinken".to_string(), 13.37);
    assert!(expected == p.parse_line("1 belegtes Brot mit Schinken at 13.37".to_owned()));
}

#[test]
fn on_multiple_at_it_should_split_at_the_last_one() {
    let p = Parser::new();
    let expected = Item::new("1 belegtes Brot macht satt".to_string(), 13.37);
    assert!(expected == p.parse_line("1 belegtes Brot macht satt at 13.37".to_owned()));
}

#[test]
#[should_panic]
fn it_should_panic_on_invalid_floats() {
    let p = Parser::new();
    p.parse_line("1 belegtes Brot mit Schinken at 13,37".to_owned());
}

#[test]
#[should_panic]
fn it_should_panic_on_invalid_floats_2() {
    let p = Parser::new();
    p.parse_line("1 belegtes Brot mit Schinken at deizehn euro siebenunddreißig".to_owned());
}

#[test]
#[should_panic]
fn it_should_panic_when_not_finding_an_at() {
    let p = Parser::new();
    p.parse_line("1 belegtes Brot mit Schinken 13.37".to_owned());
}
