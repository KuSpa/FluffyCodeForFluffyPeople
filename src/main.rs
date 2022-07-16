use regex::Regex;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide a filename");
    } else if args.len() == 2 {
        drive(args.get(1).unwrap());
    } else {
        // >2 params given
        println!("Please provide a ONLY filename");
    }
}

fn drive(filename: &str) {
    let file = File::open(filename).expect(format!("Could not read file: {:?}", filename).as_str());
    let parser = Parser::new();
    if let Some((total_price, total_tax)) = parser
        .parse(&file)
        .into_iter()
        .map(|item| TaxCalculator::new(item))
        .map(|item| {
            // Print and prepare reduction
            let tax = item.calculate_tax();
            println!("{}: {:.2}", item.name(), tax + item.price());
            (item.price(), tax)
        })
        .reduce(|(price_a, tax_a), (price_b, tax_b)| (price_a + price_b, tax_a + tax_b))
    {
        println!("Sales Taxes: {:.2}", total_tax);
        println!("Total: {:.2}", total_price + total_tax);
    }
}

struct Item {
    name: String,
    price: f64,
}

impl Item {
    fn new(name: String, price: f64) -> Self {
        Item {
            name: name,
            price: price,
        }
    }
}

struct TaxCalculator {
    item: Item,
    strategies: Vec<Box<dyn TaxStrategy>>,
}

impl TaxCalculator {
    fn new(item: Item) -> Self {
        let all: Vec<Box<dyn TaxStrategy>> =
            vec![Box::new(ImportTaxStrategy), Box::new(BasicSalesTaxStrategy)];
        let strategies = all
            .into_iter()
            .filter(|t| t.is_relevant_for(&item))
            .map(|t| t)
            .collect();

        TaxCalculator { item, strategies }
    }

    fn name<'a>(&'a self) -> &'a str {
        &self.item.name
    }

    fn price(&self) -> f64 {
        self.item.price
    }

    fn calculate_tax(&self) -> f64 {
        let unrounded_tax: f64 = self
            .strategies
            .iter()
            .map(|strat| strat.apply(self.item.price))
            .sum();

        (unrounded_tax * 20.).ceil() / 20.
    }
}

trait TaxStrategy {
    fn apply(&self, price: f64) -> f64;

    fn is_relevant_for(&self, item: &Item) -> bool;
}

#[derive(Clone, Copy)]
struct ImportTaxStrategy;
impl TaxStrategy for ImportTaxStrategy {
    fn is_relevant_for(&self, item: &Item) -> bool {
        item.name.contains("imported")
    }

    fn apply(&self, price: f64) -> f64 {
        price * 0.05
    }
}

#[derive(Clone, Copy)]
struct BasicSalesTaxStrategy;
impl TaxStrategy for BasicSalesTaxStrategy {
    fn is_relevant_for(&self, item: &Item) -> bool {
        !(item.name.contains("book")
            || item.name.contains("chocolate")
            || item.name.contains("pills"))
    }

    fn apply(&self, price: f64) -> f64 {
        price * 0.1
    }
}

struct Parser;
impl Parser {
    fn new() -> Self {
        Parser {}
    }

    fn parse(&self, file: &File) -> Vec<Item> {
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
