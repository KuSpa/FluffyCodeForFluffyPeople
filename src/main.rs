mod parser;
mod shoppinglist;
mod tax;

use parser::Parser;
use shoppinglist::ShoppingList;
use std::{env, fs::File};

fn main() {
    let filename = get_file_name();
    let file = File::open(&filename).expect(format!("Could not read file: {:?}", &filename).as_str());
    let parser = Parser::new();
    let items = parser.parse(&file);
    let shoppinglist = ShoppingList::from(items);
    shoppinglist.print();
}

fn get_file_name()->String{
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("Please provide a filename");
    } else if args.len() == 2 {
        return args.get(1).unwrap().to_owned();
    } else {
        // >2 params given
        panic!("Please provide a ONLY filename");
    }
}