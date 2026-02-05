use chrono:: prelude::*;
use polars_core::prelude::DataType;
use polars::prelude::*;

fn load_csv(csv_file: &str) -> LazyFrame {
    let lazy_frame = LazyCsvReader::new(csv_file)
        .has_header(true)
        .finish()?;

    match lazy_frame {
        Ok(lf) => {
            println!("Successfully loaded: {}", csv_file);
            
            // returns lazy data frame.
            lf
        },
        Err(e) => {
            eprintln!("Error: Could not open the file found in {}! Details: {},", csv_file, e);
            
            // returns empty lazy data frame.
            DataFrame::empty().lazy()
        }
    }
    
}

fn example() {
    let mut df: DataFrame = df!(
    "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
    "birthdate" => [
        NaiveDate::from_ymd_opt(1997, 1, 10).unwrap(),
        NaiveDate::from_ymd_opt(1985, 2, 15).unwrap(),
        NaiveDate::from_ymd_opt(1983, 3, 22).unwrap(),
        NaiveDate::from_ymd_opt(1981, 4, 30).unwrap(),
    ],
    "weight" => [57.9, 72.5, 53.6, 83.1],  // (kg)
    "height" => [1.56, 1.77, 1.65, 1.75],  // (m)
    )
    .unwrap();
    println!("{df}");
}
fn main() {

    // lazy data frame to manipulate
    let data = load_csv("data.csv");
    println!("{}", data);
}
