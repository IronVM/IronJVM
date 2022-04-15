extern crate core;

use std::fs::File;

use ironjvm_cfparser::ClassFileParser;

#[test]
fn hello_world() {
    let result = File::open("../test_classes/HelloWorld.class");
    if let Err(error) = result {
        panic!("failed to open classfile: {error}");
    }

    let file = result.unwrap();
    let parser = ClassFileParser::new(file);
    let result = parser.parse();
    if let Err(error) = result {
        panic!("failed to parse classfile: {error:?}");
    }
}
