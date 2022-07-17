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
        item.name.to_lowercase().contains("import")
    }

    fn apply(&self, price: f64) -> f64 {
        price * 0.05
    }
}

#[derive(Clone, Copy)]
struct BasicSalesTaxStrategy;
impl TaxStrategy for BasicSalesTaxStrategy {
    fn is_relevant_for(&self, item: &Item) -> bool {
        !(item.name.to_lowercase().contains("book")
            || item.name.contains("chocolate")
            || item.name.contains("pills"))
    }

    fn apply(&self, price: f64) -> f64 {
        price * 0.1
    }
}

/********************************************************************
 *  TESTING THE CORRECT CHOICE OF TAX STRATEGIES
 ********************************************************************/

#[test]
fn base_tax_and_nothing_else() {
    let item = Item::new("KRANPLÄTZE MÜSSEN VERDICHTET WERDEN".to_owned(), 10.);
    assert!(TaxCalculator::new(item).calculate_tax() == 1.)
}

#[test]
fn no_base_tax_and_nothing_else() {
    let item = Item::new("Life is a box of chocolate".to_owned(), 10.);
    assert!(TaxCalculator::new(item).calculate_tax() == 0.)
}

#[test]
fn base_tax_and_import() {
    let item = Item::new("WEIL DIE AM LEBEN VORBEIIMPORTIEREN".to_owned(), 10.);
    let real = TaxCalculator::new(item).calculate_tax();
    print!("{}", real);
    assert!(real == 1.5)
}

#[test]
fn no_base_tax_and_import() {
    let item = Item::new("import pills by pressing on that button".to_owned(), 10.);
    assert!(TaxCalculator::new(item).calculate_tax() == 0.5)
}

#[test]
fn ceil_correctly() {
    let item = Item::new("KRANPLÄTZE MÜSSEN VERDICHTET WERDEN".to_owned(), 10.11);
    // make sure that: x.xxX <- last one is zero to avoid formatting problems
    // trunc to compare floats properly

    // Tax is 1.011 -> 1.05
    assert!((TaxCalculator::new(item).calculate_tax() * 1000.).trunc() == 1050.)
}

#[test]
fn ceil_correctly_2() {
    let item = Item::new("KRANPLÄTZE MÜSSEN VERDICHTET WERDEN".to_owned(), 10.011);
    // make sure that: x.xxX <- last one is zero to avoid formatting problems
    // trunc to compare floats properly
    assert!((TaxCalculator::new(item).calculate_tax() * 1000.).trunc() == 1050.)
}
