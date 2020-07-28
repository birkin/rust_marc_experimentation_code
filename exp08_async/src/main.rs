extern crate glob;  // <https://docs.rs/glob/0.3.0/glob/>
extern crate marc; // <https://github.com/blackbeam/rust-marc>

use chrono::{DateTime, Local};
use glob::glob;
use marc::*;

// use std::time::{Duration, Instant};
// use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::{Instant};
use std::time;
use std::thread::sleep;



use tokio::io;
use tokio::sync::mpsc;


/*
    ASCII 1D hex
    - see "COMPONENTS OF BIBLIOGRAPHIC RECORDS" section of <http://www.loc.gov/marc/bibliographic/bdintro.html>
    - see <https://doc.rust-lang.org/std/primitive.char.html>
 */
const RECORD_TERMINATOR: u8 = 0x1D;

#[tokio::main]
async fn main() -> io::Result<()> {
    let start_now = time::Instant::now();
    let first_start_time = Instant::now();

    // -- set non-loop vars
    let source_files_dir: String = "./source_files".to_string();
    let pattern: String = format!( "{}/*.mrc", source_files_dir );
    // println!("pattern, ``{:?}``", pattern);
    let output_filepath: String = "./output.txt".to_string();

    // -- set vars for loop
    let title_field_tag: String = "245".to_string();
    let title_subfield_main_identifier: String = "a".to_string();
    let title_subfield_remainder_identifier: String = "b".to_string();
    let bib_field_tag: String = "907".to_string();
    let bib_subfield_bib_identifier: String = "a".to_string();

    // -- get paths
    let paths: glob::Paths = glob( &pattern ).unwrap_or_else( |err| {
        panic!("could not glob the pattern; error, ``{}``", err);
    });

    let mut marc_filepaths: Vec<String> = Vec::new();
    for path in paths {  // // path type check yields: found enum `std::result::Result<std::path::PathBuf, glob::GlobError>`
        let path_buf: std::path::PathBuf = path.unwrap();
        let path_str: &str = path_buf.to_str().unwrap_or_else( || {panic!("problem converting PathBuf obj to &str  -- ``{:?}``");} );
        let marc_filepath: String = path_str.into();
        marc_filepaths.push( marc_filepath )
    }

    // -- clear output file
    // let mut f: std::fs::File = File::create( &output_filepath ).unwrap();
    fs::File::create( &output_filepath ).unwrap_or_else( |err| {
        panic!( "problem initializing the output file; error, ``{}``", err );
    });

    // -- get an append file-handler that i'll pass to the writer functions
    let fappend = fs::OpenOptions::new()
        .append(true)
        .open( &output_filepath )
        .unwrap();

    let (tx, mut rx) = mpsc::channel( 100 );
    // -- loop through paths
    // let mut file_counter: i32 = 0;
    for marc_filepath in marc_filepaths {  // marc_filepath type-check yields: found struct `std::string::String`

        let mut tx = tx.clone();

        tokio::spawn( async move {
            let text_to_write: String = expensive_computation(
                &marc_filepath,
                title_field_tag.clone(),
                &title_subfield_main_identifier,
                &title_subfield_remainder_identifier,
                &bib_field_tag,
                &bib_subfield_bib_identifier ).await;
            tx.send( text_to_write ).await.unwrap();
        });

        // -- load file into marc-reader
        // let marc_records: Vec<marc::Record> = load_records( &marc_filepath );

        // -- process records
        // for rec in marc_records.iter() {  // yields: `&marc::Record<'_>`
        //     let mut title: String = "".to_string();
        //     let mut bib: String = "".to_string();
        //     // println!("\nnew record...");
        //     for field in rec.field( Tag::from(title_field_tag.as_str()) ).iter() {
        //         // process_title( field, &title_subfield_main_identifier, &title_subfield_remainder_identifier, &output_filepath );
        //         title = process_title( field, &title_subfield_main_identifier, &title_subfield_remainder_identifier, &fappend );
        //     }
        //     for field in rec.field( Tag::from(bib_field_tag.as_str()) ).iter() {
        //         // process_bib( field, &bib_subfield_bib_identifier, &output_filepath )
        //         bib = process_bib( field, &bib_subfield_bib_identifier, &fappend );
        //     }

        //     let text_to_write = format!( "title, ``{}``; bib, ``{}``", &title, &bib  );
        // }

    }  // end of `for marc_filepath in marc_filepaths {`

    println!("about to call drop");
    drop( tx );
    println!("just called drop");

    while let Some( text_to_write ) = rx.recv().await {
        // write!( &fappend, "\n\n{}", text_to_write ).unwrap();
        write_to_file( &fappend, &text_to_write )
    }


    // let all_files_duration: Duration = first_start_time.elapsed();
    println!("\n-------");
    // println!( "\nfiles processed, ``{:?}``", file_counter );
    // let all_files_duration: f32 = first_start_time.elapsed().as_secs_f32();
    // println!( "{}", format!("\nall-files-elapsed-time, ``{:?}`` seconds\n", all_files_duration) );
    let all_files_duration_in_minutes: f32 = first_start_time.elapsed().as_secs_f32() / 60.0;
    println!( "{}", format!("\nall-files-elapsed-time, ``{:?}`` minutes\n", all_files_duration_in_minutes) );

    Ok( () )

}  // end `fn main() {`



fn write_to_file( mut fappend: &std::fs::File, text_to_write: &str ) {
    write!( fappend, "\n\n{}", text_to_write ).unwrap();
}



async fn expensive_computation(
    marc_filepath: &str,
    title_field_tag: String,
    title_subfield_main_identifier: &str,
    title_subfield_remainder_identifier: &str,
    bib_field_tag: &str,
    bib_subfield_bib_identifier: &str ) -> String {


    // -- load file into marc-reader
    let marc_records: Vec<marc::Record> = load_records( marc_filepath );

    // -- process records
    let mut text_to_write: String = "".to_string();
    for rec in marc_records.iter() {  // yields: `&marc::Record<'_>`
        let mut title: String = "".to_string();
        let mut bib: String = "".to_string();
        // println!("\nnew record...");
        for field in rec.field( Tag::from(title_field_tag.as_str()) ).iter() {
            // process_title( field, &title_subfield_main_identifier, &title_subfield_remainder_identifier, &output_filepath );
            title = process_title( field, title_subfield_main_identifier, &title_subfield_remainder_identifier );
        }
        for field in rec.field( Tag::from(bib_field_tag) ).iter() {
            // process_bib( field, &bib_subfield_bib_identifier, &output_filepath )
            bib = process_bib( field, &bib_subfield_bib_identifier );
        }

        text_to_write = format!( "{}\n\ntitle, ``{}``; bib, ``{}``", text_to_write, &title, &bib  );
        // text_to_write = format!( "title, ``{}``; bib, ``{}``", &title, &bib  );
    }


    // let text_to_write: String = "testing 1, 2, 3".to_string();

    text_to_write
}


// async fn expensive_computation( input: u32, start_now: time::Instant ) -> String {
//     let now = time::Instant::now();  // for elapsed-time
//     let local_time: DateTime<Local> = Local::now();
//     println!( "\nstarting expensive_computation at, ``{:?}`` on thread, ``{:?}``", local_time.to_rfc3339(), std::thread::current().id() );

//     sleep( time::Duration::from_secs(2) );
//     let msg: String = format!( "that_took, ``{:?}`` -- for a total elapsed time of, ``{:?}`` -- on thread, ``{:?}``", now.elapsed(), start_now.elapsed(), std::thread::current().id() ).to_string();
//     println!( "msg, {:?}", msg );

//     let text_to_write: String = format!( "the result of computation {}", input );
//     text_to_write
// }



// fn process_title( field: &marc::Field<'_>, title_subfield_main_identifier: &str, title_subfield_remainder_identifier: &str, output_filepath: &str ) {
fn process_title( field: &marc::Field<'_>, title_subfield_main_identifier: &str, title_subfield_remainder_identifier: &str ) -> String {

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
    // println!("full_title, ``{:?}``", final_title);

    final_title

    // write!( fappend, "{}", &final_title ).unwrap();

}


// fn process_bib( field: &marc::Field<'_>, bib_subfield_bib_identifier: &str, output_filepath: &str ) {
fn process_bib( field: &marc::Field<'_>, bib_subfield_bib_identifier: &str ) -> String {

    // println!( "all_bib_subfields, ``{:?}``", field.get_data::<str>() );
    let mut raw_bib: String = "".to_string();

    for subfield in field.subfield( Identifier::from(bib_subfield_bib_identifier) ).iter() {
        raw_bib = format!( "{}", subfield.get_data::<str>() );
        // println!( "bib_subfield, ``{}``", subfield.get_data::<str>() );
        // println!("bib_subfield, ``{:?}``", raw_bib );
        // let bib_url: String = make_bib_url( )
    }

    // make_bib_url( &raw_bib );
    let bib_url: String = make_bib_url( &raw_bib );

    // write!( fappend, "\n{}\n\n", &bib_url ).unwrap();

    bib_url

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
    // println!( "bib_url, ``{:?}``", bib_url );
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
    let file = match fs::File::open(&path) {
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
