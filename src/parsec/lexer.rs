// import std io mod - read from std io
use std::io;
use std::path::Path;
use csv::Reader;
use std::error::Error;

// verify that file exists
fn validate_and_read_csv(csv_file: &str) -> bool {
    let path = Path::new(csv_file);
    let result = false;
    // check path and file
    if !path.exists() {
        eprintln!("path doesnt exist. {}", path.display());
        return result;
    }
    if !path.is_file() {
        eprintln!("path isnt a file. {}", path.display());
        return result;
    }
    
    if path.extension().unwrap() == "csv" {
        println!("valid file extension. {}", path.display());
        return !result;
    }
    return result;
}

fn read_input_file() -> String {
    let mut file_buffer = String::new();

    io::stdin()
        .read_line(&mut file_buffer)
        .expect("Failed to read line.");

    let trim_file_buffer = file_buffer.trim().to_string();

    println!("File name entered: {}", trim_file_buffer);
    trim_file_buffer
}

pub fn scan() -> Result<(), Box<dyn Error>>{
    // read input file.
    let file_path;
    println!("Enter csv file to parsec: ");
    file_path = read_input_file();
    
    if !validate_and_read_csv(&file_path) {
        ()
    }
    
    let mut csv_table = Reader::from_path(file_path)?;

    {
        let table_headers = csv_table.headers();

        println!("{:#?}", table_headers);

    }
    for result in csv_table.records() {
        let record = result.expect("expected a record");
        println!("record: {:?}", record);
    }
    // for result in csv_parser.records() {
    //    let record = result.expect("a csv record");
    //    println!("{:?}", record);
    //}
    Ok(())

}
