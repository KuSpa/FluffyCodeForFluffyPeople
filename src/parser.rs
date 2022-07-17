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

    pub fn parse(&self, file: &File) -> Vec<Item> {
        let file_buffer = BufReader::new(file);

        file_buffer
            .lines()
            .map(|line_res| line_res.expect("Error reading line"))
            .map(|line| {
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
            })
            .collect()
    }
}

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
