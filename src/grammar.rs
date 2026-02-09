// enum with variants
#[derive(Debug)]
enum Currency {
    CENTS,
    Dollar(u32),
    Cent(u8),
    Values(String),
}
// returns currency variant CENTS
fn bill() -> Currency {
    let one = Currency::CENTS;
    one
}
// returns integer type (-2^127)
fn coin(variant_c: Currency) -> i128 {
    1i128
}
fn print_currency (c: i128) -> String {
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
