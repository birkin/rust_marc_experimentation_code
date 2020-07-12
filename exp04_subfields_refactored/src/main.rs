extern crate marc; // <https://github.com/blackbeam/rust-marc>

use marc::*;

use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

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

    // -- output title and bib
    let title_field_tag: String = "245".to_string();
    let title_subfield_main_identifier: String = "a".to_string();
    let title_subfield_remainder_identifier: String = "b".to_string();

    let bib_field_tag: String = "907".to_string();
    let bib_subfield_bib_identifier: String = "a".to_string();

    for rec in marc_records.iter() {
        println!("\nnew rec...");

        for field in rec.field( Tag::from(title_field_tag.as_str()) ).iter() {
            println!( "all_title_subfields, ``{}``", field.get_data::<str>() );
            let mut title: String = "".to_string();
            let mut final_title: String = "".to_string();
            for subfield in field.subfield( Identifier::from(title_subfield_main_identifier.as_str()) ).iter() {
                title = format!( "{}", subfield.get_data::<str>() );
                // println!( "``- {}``", subfield.get_data::<str>() );
                // println!("title: ``{:?}``", title);
            }
            for subfield in field.subfield( Identifier::from(title_subfield_remainder_identifier.as_str()) ).iter() {
                let subtitle: String = format!( "{}", subfield.get_data::<str>() );
                println!("subtitle, ``{:?}``", subtitle );
                if subtitle.chars().count() > 1 {
                    final_title = format!( "{} {}", &title, &subtitle );
                }
                // println!( "``--- subtitle --- {}``", subfield.get_data::<str>() );
            }
            if final_title.chars().count() == 0 {
                final_title = format!( "{}", &title );
            }
            println!("final_title, ``{:?}``", final_title);
        }

        for field in rec.field( Tag::from(bib_field_tag.as_str()) ).iter() {
            println!( "all_bib_subfields, ``{:?}``", field.get_data::<str>() );
            for subfield in field.subfield( Identifier::from(bib_subfield_bib_identifier.as_str()) ).iter() {
                println!( "bib_subfield, ``{}``", subfield.get_data::<str>() );
            }

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
