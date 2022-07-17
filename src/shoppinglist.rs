use crate::{parser::Item, tax::TaxCalculator};

pub struct ShoppingList {
    // excluding tax
    total_price: f64,
    total_tax: f64,
    items: Vec<FinalItem>,
}

impl ShoppingList {
    pub fn print(&self) {
        for item in self.items.iter() {
            println!("{}: {:.2}", item.name, item.tax + item.price);
        }
        println!("Sales Taxes: {:.2}", self.total_tax);
        println!("Total: {:.2}", self.total_price + self.total_tax);
    }
}

impl From<Vec<Item>> for ShoppingList {
    fn from(raw_items: Vec<Item>) -> Self {
        let items: Vec<FinalItem> = raw_items
            .into_iter()
            .map(|item| FinalItem::from(TaxCalculator::new(item)))
            .collect();

        let (total_price, total_tax): (f64, f64) = items
            .iter()
            .map(|item| (item.price, item.tax))
            .reduce(|(price_a, tax_a), (price_b, tax_b)| (price_a + price_b, tax_a + tax_b))
            .unwrap_or_default();

        Self {
            items,
            total_price,
            total_tax,
        }
    }
}

pub struct FinalItem {
    name: String,
    price: f64,
    tax: f64,
}

impl From<TaxCalculator> for FinalItem {
    fn from(item: TaxCalculator) -> Self {
        Self {
            name: item.name().to_owned(),
            price: item.price(),
            tax: item.calculate_tax(),
        }
    }
}

// We already established that the conversion of line->Item->TaxCalculator->Tax is correct
// Thus, we only add a small test that tests with a variety of items and don't test every scenario for itself
#[test]
fn it_should_calculate_the_correct_taxes() {
    let items: Vec<Item> = vec![
        Item::new("nothing but chocolate".to_owned(), 10.), // no tax,
        Item::new("base tax".to_owned(), 100.),             // 10. tax
        Item::new("base and import".to_owned(), 1000.),     //150. tax
        Item::new("nothing pills and import".to_owned(), 10000.), // 500. tax
    ];

    let list = ShoppingList::from(items);
    assert!(list.total_price == 11110.);
    assert!(list.total_tax == 660.);
}
