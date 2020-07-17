extern crate glob;  // <https://docs.rs/glob/0.3.0/glob/>
extern crate marc; // <https://github.com/blackbeam/rust-marc>

use glob::glob;
use marc::*;

use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
// use std::time::{Duration, Instant};
use std::time::{Instant};


/*
    ASCII 1D hex
    - see "COMPONENTS OF BIBLIOGRAPHIC RECORDS" section of <http://www.loc.gov/marc/bibliographic/bdintro.html>
    - see <https://doc.rust-lang.org/std/primitive.char.html>
 */
const RECORD_TERMINATOR: u8 = 0x1D;


fn main() {

    let first_start_time = Instant::now();

    let file_download_dir: String = "./source_files".to_string();
    let pattern: String = format!( "{}/*.mrc", file_download_dir );
    // println!("pattern, ``{:?}``", pattern);

    let paths: glob::Paths = glob( &pattern ).unwrap_or_else( |err| {
        panic!("could not glob the pattern; error, ``{}``", err);
    });

    /*
        TODO:
        - create a vector of path-strings here so I don't have to do conversions each time...
            ...or better yet, figure out how to more simply get the path as a String.
        - if count of paths < 1, return error.
     */

    for path_buf_result in paths {  // path_buf_result yields: found enum `std::result::Result<std::path::PathBuf, glob::GlobError>`

        println!("\nnew file...");
        let file_start_time = Instant::now();

        let path_buf: std::path::PathBuf = path_buf_result.unwrap();
        let path_str: &str = path_buf.to_str().unwrap_or_else( || {panic!("problem converting PathBuf obj to &str  -- ``{:?}``");} );
        let marc_path: String = path_str.into();
        println!( "marc_path, ``{:?}``", marc_path );

        // -- load
        let marc_records: Vec<marc::Record> = load_records( &marc_path );
        // println!("first marc_record, ``{:?}``", marc_records[0]);

        // -- setup vars
        let title_field_tag: String = "245".to_string();
        let title_subfield_main_identifier: String = "a".to_string();
        let title_subfield_remainder_identifier: String = "b".to_string();

        let bib_field_tag: String = "907".to_string();
        let bib_subfield_bib_identifier: String = "a".to_string();

        // -- process records
        for rec in marc_records.iter() {  // yields: `&marc::Record<'_>`

            println!("\nnew record...");

            for field in rec.field( Tag::from(title_field_tag.as_str()) ).iter() {
                process_title( field, &title_subfield_main_identifier, &title_subfield_remainder_identifier );
            }

            for field in rec.field( Tag::from(bib_field_tag.as_str()) ).iter() {
                process_bib( field, &bib_subfield_bib_identifier )
            }

        }

        // let file_duration: Duration = file_start_time.elapsed();
        let file_duration: f32 = file_start_time.elapsed().as_secs_f32();
        println!( "{}", format!("\nfile-elapsed-time, ``{:?}``", file_duration) );

    }  // end of for path_buf_result in paths {

    // let all_files_duration: Duration = first_start_time.elapsed();
    let all_files_duration: f32 = first_start_time.elapsed().as_secs_f32();
    println!( "{}", format!("\nall-files-elapsed-time, ``{:?}``\n", all_files_duration) );

}


fn process_title( field: &marc::Field<'_>, title_subfield_main_identifier: &str, title_subfield_remainder_identifier: &str ) {

    // println!( "all_title_subfields, ``{}``", field.get_data::<str>() );
    let mut title: String = "".to_string();
    let mut final_title: String = "".to_string();

    for subfield in field.subfield( Identifier::from(title_subfield_main_identifier) ).iter() {
        title = format!( "{}", subfield.get_data::<str>() );
        // println!( "``- {}``", subfield.get_data::<str>() );
        // println!("title: ``{:?}``", title);
    }
    for subfield in field.subfield( Identifier::from(title_subfield_remainder_identifier) ).iter() {
        let subtitle: String = format!( "{}", subfield.get_data::<str>() );
        // println!("subtitle, ``{:?}``", subtitle );
        if subtitle.chars().count() > 1 {
            final_title = format!( "{} {}", &title, &subtitle );
        }
        // println!( "``--- subtitle --- {}``", subfield.get_data::<str>() );
    }
    if final_title.chars().count() == 0 {
        final_title = format!( "{}", &title );
    }
    println!("full_title, ``{:?}``", final_title);

}


fn process_bib( field: &marc::Field<'_>, bib_subfield_bib_identifier: &str ) {

    // println!( "all_bib_subfields, ``{:?}``", field.get_data::<str>() );
    let mut raw_bib: String = "".to_string();

    for subfield in field.subfield( Identifier::from(bib_subfield_bib_identifier) ).iter() {
        raw_bib = format!( "{}", subfield.get_data::<str>() );
        // println!( "bib_subfield, ``{}``", subfield.get_data::<str>() );
        // println!("bib_subfield, ``{:?}``", raw_bib );
        // let bib_url: String = make_bib_url( )
    }

    make_bib_url( &raw_bib );

}


fn make_bib_url( raw_bib: &str ) -> String {
    let end: usize = raw_bib.len();
    // println!("end, ``{:?}``", end );
    let start: usize = 1;
    let bib_a: String = ( &raw_bib[start..end ]).to_string();
    // println!("bib_a, ``{:?}``", bib_a );

    let end_2: usize = &bib_a.len() - 1;
    let start_2: usize = 0;
    let bib_b: String = ( &bib_a[start_2..end_2 ]).to_string();

    // let bib_url: String = "foo".to_string();
    let bib_url: String = format!( "https://search.library.brown.edu/catalog/{}", &bib_b );
    println!( "bib_url, ``{:?}``", bib_url );
    bib_url
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
