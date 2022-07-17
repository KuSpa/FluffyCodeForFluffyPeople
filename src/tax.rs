use crate::parser::Item;

pub struct TaxCalculator {
    item: Item,
    strategies: Vec<Box<dyn TaxStrategy>>,
}

impl TaxCalculator {
    pub fn new(item: Item) -> Self {
        let all: Vec<Box<dyn TaxStrategy>> =
            vec![Box::new(ImportTaxStrategy), Box::new(BasicSalesTaxStrategy)];
        let strategies = all
            .into_iter()
            .filter(|t| t.is_relevant_for(&item))
            .map(|t| t)
            .collect();

        TaxCalculator { item, strategies }
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &self.item.name
    }

    pub fn price(&self) -> f64 {
        self.item.price
    }

    pub fn calculate_tax(&self) -> f64 {
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
