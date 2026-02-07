//! this is my first rust crate :^C
// this is documentation example for further reference.
//
// check rustdoc : doc.rust-lang.org/rustdoc
//

// 

/// foo is function -
/// rustdoc defaults to generating documentation for only public functions
pub fn foo() {}

// pub is showing a  link to the foo page in the docs. 
// see <path>/doc/docs/
// after running... rustdoc src/lib.rs --crate name docs
// optional: running cargo doc makes easier to generate docs.
// cargo doc --open

// look the '///' syntax for writing function comments in the
// create docs. this will document the item present after it.
// '//!' is used for inner documentation 
// '//!' will document an entire crate 


