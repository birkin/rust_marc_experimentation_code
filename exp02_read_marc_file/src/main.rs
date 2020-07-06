extern crate marc; // <https://github.com/blackbeam/rust-marc>

use marc::*;

use std::fs::File;
use std::path::Path;
// use std::io::{BufRead, BufReader, Read};
use std::io::{BufRead, BufReader};
// use std::error::Error;


// use std::fmt::Debug;
// use std::fmt::Display;


const RECORD_TERMINATOR: u8 = 0x1D;



fn main() {

    // -- get marc file path
    let marc_path: String = "./source_files/sierra_export_0726.mrc".to_string();
    println!( "marc_path, ``{:?}``", marc_path);

    // -- load
    let marc_records: Vec<marc::Record> = load_records( marc_path );
    println!("marc_records, ``{:?}``", marc_records);

    // -- output title
    let title_field_tag: String = "245".to_string();

    for rec in marc_records.iter() {
        println!("\nnew rec...");
        // for field in rec.field(Tag::from(field_tag.as_str())).iter() {
        for field in rec.field( Tag::from(title_field_tag.as_str()) ).iter() {
            println!("{}", field.get_data::<str>());
        }

    }

}


fn load_records( filename: String ) -> Vec< marc::Record<'static> > {
    // -- using marc_cli to grok
    let mut result_vector: Vec<marc::Record> = Vec::new();
    let mut buffer = Vec::new();

    let file_path = filename.clone();

    let path = Path::new(file_path.as_str());
    let display = path.display();

    let file = match File::open(&path) {
        // Err(why) => panic!( "Couldn't open {}: {}", display, why.description() ),  // why.description() errored until I added ``use std::error::Error;``
        Err(why) => panic!( "Couldn't open {}: {}", display, why.to_string() ),
        Ok(file) => file,
    };

    let mut file = BufReader::new(file);
    while file.read_until(RECORD_TERMINATOR, &mut buffer).unwrap() != 0 {
        match marc::Record::from_vec(buffer.clone()) {
            Err(_) => (),
            Ok(record) => result_vector.push(record.clone()),
        }

        buffer.clear();
    }

    return result_vector;

}
