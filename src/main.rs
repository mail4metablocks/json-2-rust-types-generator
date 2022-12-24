extern crate json_schema_to_rust;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: json-schema-to-rust <schema-file>");
        return;
    }

    let schema_file = &args[1];
    let mut schema = String::new();
    File::open(schema_file)
        .unwrap()
        .read_to_string(&mut schema)
        .unwrap();

    let generated_types = json_schema_to_rust::generate_types(&schema);
    for t in generated_types {
        println!("{}", t);
    }
}
