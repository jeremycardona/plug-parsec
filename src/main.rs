use crate::parsec::lexer;
pub mod parsec;

fn main() {
    // let convert_values;
    
    // scan csv input file.
    let scan_input_file = lexer::scan();
    scan_input_file.expect("expected soemthing")
}
