extern crate marc; // <https://github.com/blackbeam/rust-marc>

use marc::*;

use std::fs::File;
use std::path::Path;
// use std::io::{BufRead, BufReader, Read};
use std::io::{BufRead, BufReader};
// use std::error::Error;


// use std::fmt::Debug;
// use std::fmt::Display;


/*
    ASCII 1D hex
    - see "COMPONENTS OF BIBLIOGRAPHIC RECORDS" section of <http://www.loc.gov/marc/bibliographic/bdintro.html>
    - see <https://doc.rust-lang.org/std/primitive.char.html>
 */
const RECORD_TERMINATOR: u8 = 0x1D;



fn main() {

    // -- get marc file path
    let marc_path: String = "./source_files/sierra_export_0726.mrc".to_string();
    println!( "marc_path, ``{:?}``", marc_path);

    // -- load
    let marc_records: Vec<marc::Record> = load_records( &marc_path );
    println!("first marc_record, ``{:?}``", marc_records[0]);

    // -- output title
    let title_field_tag: String = "245".to_string();
    for rec in marc_records.iter() {
        println!("\nnew rec...");
        for field in rec.field( Tag::from(title_field_tag.as_str()) ).iter() {
            println!( "{}", field.get_data::<str>() );
        }
    }

}


fn load_records( file_path: &str ) -> Vec< marc::Record<'static> > {

    /* marc_cli was helpful figuring out how to do this */

    // create the return Vec
    let mut result_vector: Vec<marc::Record> = Vec::new();

    // create path-object to pass to file-handler
    let path = Path::new( file_path );
    let error_path_display = path.display();

    // access the file
    let file = match File::open(&path) {
        Err(why) => panic!( "Couldn't open {}: {}", error_path_display, why.to_string() ),
        Ok(file) => file,
    };

    /*
        <https://doc.rust-lang.org/std/io/struct.BufReader.html>

        "...A BufReader<R> performs large, infrequent reads on the underlying Read and maintains an in-memory buffer of the results.
        BufReader<R> can improve the speed of programs that make small and repeated read calls to the same file or network socket...""
     */

    let mut buf_reader = BufReader::new( file );
    let mut marc_record_buffer = Vec::new();  // the buffer where the marc-record-segment will be stored

    while buf_reader.read_until( RECORD_TERMINATOR, &mut marc_record_buffer ).unwrap() != 0 {
        match marc::Record::from_vec(marc_record_buffer.clone()) {
            Err(_) => (),
            Ok(record) => result_vector.push(record.clone()),
        }

        marc_record_buffer.clear();
    }

    return result_vector;
}
