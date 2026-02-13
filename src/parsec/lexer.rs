use std::io;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use csv::Reader;
use serde::Deserialize;
use super::grammar::Currency;
/// type represents each row in csv. used in sarde deserialization
// #[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, Deserialize)]
struct Record {
    /// Gross Pay,Deductions,Federal Income Tax,Social Security Tax,Medicare Tax,State Income Tax
    #[serde(alias = "Gross Pay", alias = "gross_pay")]
    gross_pay: String,
    #[serde(alias = "Deductions", alias = "deductions")]
    deductions: String, 
    #[serde(alias = "Federal Income Tax", alias = "federal_income_tax")]
    federal_income_tax: String,
    #[serde(alias = "Social Security Tax", alias = "social_security_tax")]
    social_security_tax: String,
    #[serde(alias = "Medicare Tax", alias = "medicare_tax")]
    medicare_tax: String,
    #[serde(alias = "State Income Tax", alias = "state_income_tax")]
    state_income_tax: String,
}
impl Record {
    /// Return an iterator of (name, value) pairs
    fn fields(&self) -> Vec<(&str, &String)> {
        vec![
            ("gross_pay", &self.gross_pay),
            ("deductions", &self.deductions),
            ("federal_income_tax", &self.federal_income_tax),
            ("social_security_tax", &self.social_security_tax),
            ("medicare_tax", &self.medicare_tax),
            ("state_income_tax", &self.state_income_tax),
        ]
    }
}
/// advanced type (TODO: study https://doc.rust-lang.org/book/ch20-03-advanced-types.html)
type Rec = Record;
/// strong typed data structure.
struct Data {
    gross_pay: Currency,
    deductions: Currency,
    federal_income_tax: Currency,
    social_security_tax: Currency,
    medicare_tax: Currency,
    state_income_tax: Currency,
}
/// verify that file exists
fn validate_and_read_csv(csv_file: &str) -> bool {
    let path = Path::new(csv_file);
    let result = false;
    /// check path and file
    if !path.exists() {
        eprintln!("path doesnt exist. {}", path.display());
        return result;
    }
    if !path.is_file() {
        eprintln!("path isnt a file. {}", path.display());
        return result;
    }

    // check extension
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

//
fn scan() -> Result<Reader<File>, csv::Error> {
    /// read input file.
    println!("Enter csv file to parsec: ");
    let path_file = read_input_file();

    /// this should return something when
    /// the file path is invalid.
    /*
    if !validate_and_read_csv(&path_file) {
        ()
    }
    */

    let csv_table = Reader::from_path(path_file);
    csv_table
    /*
        {
            let table_headers = csv_table.headers();
            println!("{:#?}", table_headers);

        }
        let record;
        for result in csv_table.records() {
            record = result.expect("expected a string record");
            println!("record: {:?}", record);
        }
        Ok((record))
    */
}

/// this function might fail if the string is not safely formatted. eg. 0.00
fn transform(money: String, money_length: usize) {
    /// conversion is used for the system
    /// to convert dollars to cents.
    /// WARNING: typo in conversion array will damage conversion.
    ///
    /// 0 - ten thousand
    /// 1 - hundred thousand
    /// 2 - thousands
    /// 3 - hundred
    /// 4 - ten
    /// 5 - one again but different
    /// 6 - ten cents
    /// 7 - ones cents
    let conversion_units = [
        10000000,
        1000000,
        100000,
        10000,
        1000,
        100,
        10,
        1 
    ];
    // let convert_to_benefits: Data;
    /// returns str length in bytes
    // let money_length = money.len();
    //
    /// TODO: the problem is that values ( 3, 1, 2)
    /// make the places fixed based on the money_length
    /// -which other way can we find places?
    /// 
    /// returns length in human-readable text.
    let dollars_place = money_length - 3; /// e.g. $1.99 => 0. 
    let dot_place = dollars_place + 1; /// e.g. $1.99 => 1.
    let cents_place = dot_place + 2; /// e.g. $1.99 => 3. because its up to place 3
    // NOTE: here is how i got the converse places
    let mut conversion_starting_place =  money_length - 1; // 1 here becuse dot place counts.
    println!("dollars places {} dot place{} cents places {}", dollars_place, dot_place, cents_place);
    //    let cents_length = 2;
    let mut records_fields: Vec<i64> = Vec::with_capacity(6); /// Gross Pay,Deductions,Federal Income Tax,Social Security Tax,Medicare Tax,State Income Tax
    //    let mut idx_records_fields = 0;
    /// values to assing to no_benefits_yet
//    let mut d_values: i64 = 0; 
//    let mut c_values: i64 = 0;
    let mut converted_values = 0;
    let mut integer_part = 0;
//    let mut found_dot = false;

        
    //for unit in 0..money_length {
    for (i, unit) in money.chars().enumerate() {
        let handle: i64 = match unit {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _   => -1, // This handles '.', 'A', ' ', etc.
        };
        

        // println!("units in money_len: {unit}");
        if i <= dollars_place {
            // get scalar values using encodings
            println!("{unit}");

            // now conversion
            println!("handle: {} and conv unit {}", handle, conversion_units[conversion_starting_place]);
            converted_values = handle * conversion_units[conversion_starting_place];
        }
        else if i == dot_place {
            // should start using Data here too?
            // eg Data::from_()
            // should continue next loop?
            println!("{unit}");
           
        }
        else if i > dot_place && i <= cents_place {
            // get scalar values using encodings
            println!("{unit}");
            println!("handle: {} and conv unit {}", handle, conversion_units[conversion_starting_place]);
            converted_values = handle * conversion_units[conversion_starting_place];
        }
    }
    records_fields.push(converted_values);

    for x in &records_fields {
        println!("x in records fields: {x}");
    }
    conversion_starting_place -= 1;
    /*
        if unit <= dollars_length {
            let gp = Currency.from_dollar(money[unit]);
            let deduc = Currency.from_dollar(money[unit]);
            let fit = Currency.from_dollar(money[unit]);
            let sst = Currency.from_dollar(money[unit]);
            let mdct = Currency.from_dollar(money[unit])
        }
    */
}
fn csv_to_json() -> Result<(), Box<dyn std::error::Error>> {
    let table = scan()?;
    let iter = table.into_deserialize::<Rec>();
 
    let mut record_index = 1;
    for result in iter {
        let record = result?;
        println!("record: {record_index}");
        for (name, val) in record.fields() {
            println!("{}: {}", name, val);
            let money = val.to_string();
            let money_length = money.chars().count(); /// eg. $1.99 = 4.
            transform(money, money_length);
        }
        record_index += 1;
        println!("");
    }
    // let mut to_ints_json = csv;
    // for result in iter {
        // let record: Result<T, E> = result?;

        // println!("record: {:?}", result);
    // }
//    transform(record);
    Ok(())
}
/// analyzing every string record 
pub fn parse() -> Result<(), Box<dyn std::error::Error>> {
    let res = csv_to_json();
    res

}
