// enum with variants
#[derive(Debug)]
pub enum Currency {
    INT,
    Dollar(i64), // dollars will also be stored into cents.
    Cent(i8), // cents will be stored into cents
    Decimal(char), // decimal is the separator for dollars and cents in the string type. eg:
    // "125.75"
    Values(String), // this is string value that gets read from csv sheet. 
}
impl Currency {
    // Static constructors
    //
    fn new_currency() -> Self {
        Currency::INT
    }
    // 
    fn from_dollars(dollars: i64) -> Self {
        Currency::Dollar(dollars)
    }
    //
    fn from_cents(cents: i8) -> Self {
        Currency::Cent(cents)
    }
    //
    fn from_decimal(notation: char) -> Self {
        Currency::Decimal(notation)
    }
    //
    fn from_values(values: String) -> Self {
        Currency::Values(values)
    }
}

/*
// returns currency variant CENTS
fn bill() -> Currency {
    let one = Currency::INT;
    one
}
// returns integer type (-2^127)
fn coin(variant_c: Currency) -> i128 {
    1i128
}
fn format_currency (c: i128) -> String {
    // this many pennies (aka cents, aka smallest coin) printed to ints 
    format!("pennies: {c}")
}
fn main() {
    let pennies = coin(bill());
    let one_bill = Currency::Dollar(1);
    let one_cent = Currency::Cent(1);
    let val = Currency::Values("1.01".to_string());
    let pennies = -1;
    let fmt = print_currency(pennies);
    println!("{}", fmt);
}
*/
