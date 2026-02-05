use polars::prelude::*;

fn load_csv(csv_file: &str) -> LazyFrame {
    let reader_lazy_frame = LazyCsvReader::new(PlPath::new(csv_file))
        .with_has_header(true)
        .finish();

    match reader_lazy_frame{
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
    let data_frame = frame.collect().expect("Failed to process data");

    println!("{}", data_frame);
}
