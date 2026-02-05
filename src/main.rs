use polars::prelude::*;
use std::sync::Arc;
use std::path::Path;


fn validate_and_read_csv(csv_file: &str, schema: SchemaRef) -> PolarsResult<LazyFrame> {
    let path = Path::new(csv_file);

    // check path and file
    if !path.exists() {
        return Err(PolarsError::ComputeError(
            format!("File not found: {}", csv_file).into()
        ));
    }

    if !path.is_file() {
        return Err(PolarsError::ComputeError(
            format!("Path is not a file: {}", csv_file).into()
        ));
    }

    // returns pl reader
    LazyCsvReader::new(PlPath::new(csv_file))
        .with_has_header(true)
        .with_dtype_overwrite(Some(schema))
        .finish()
}

fn load_csv(csv_file: &str) -> LazyFrame {
    // define digits and decimal places fixed to two.
    let precision = Some(38);
    let scale = Some(2);

    // data type to currency.
    let decimal_type = DataType::Decimal(precision, scale);

    // list of csv columns names
    let csv_cols = [
        "Gross Pay",
        "Deductions",
        "Federal Income Tax",
        "Social Security Tax",
        "Medicare Tax",
        "State Income Tax",
    ];

    // map schema columns name to Decimal type (currency data)
    let schema: Schema = csv_cols
        .iter()
        .map(|&name| Field::new(name.into(), decimal_type.clone()))
        .collect();
    // arc - multi threading and sharing across many cpu cores (threads, processes)
    let schema_arc = Arc::new(schema);
    // reader 
    let reader_lazy_frame = validate_and_read_csv(csv_file, schema_arc);

    match reader_lazy_frame {
        Ok(data) => {
            println!("Successfully loaded: {}", csv_file);
            
            // returns lazy data frame.
            data 
        },
        Err(e) => {
            eprintln!("Error: Could not open the file found in {}! Details: {},", csv_file, e);
            
            // returns empty lazy data frame.
            DataFrame::empty().lazy()
        }
    }
    
}

fn main() {

    // lazy data frame to manipulate
    let frame = load_csv("data.csv");

    // collect() performs IO and computations
    // expects checks for the collected results
    let data_frame = frame.collect().expect("Failed to process data");

    println!("{}", data_frame);
}
