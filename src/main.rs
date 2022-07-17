mod parser;
mod tax;

use std::{
    env,
    fs::File,
};
use parser::Parser;
use tax::TaxCalculator;

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

