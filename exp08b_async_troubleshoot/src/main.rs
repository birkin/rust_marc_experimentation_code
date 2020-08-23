extern crate glob;  // <https://docs.rs/glob/0.3.0/glob/>

use glob::glob;
use std::fs;
use tokio::io;
use tokio::sync::mpsc;


#[tokio::main]
async fn main() -> io::Result<()> {

    // -- set non-loop vars
    let source_files_dir: String = "./source_files".to_string();
    let pattern: String = format!( "{}/*.mrc", source_files_dir );
    let output_filepath: String = "./output.txt".to_string();

    // -- initialize vars for loop
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
        // let path_buf: std::path::PathBuf = path.unwrap();
        let path_buf: std::path::PathBuf = path.unwrap_or_else( |err| {
            panic!( "problem creating path_buf; error, ``{:?}``", err );
        } );
        // let path_str: &str = path_buf.to_str().unwrap_or_else( || {panic!("problem converting PathBuf obj to &str  -- ``{:?}``");} );
        let path_str: &str = path_buf.to_str().unwrap_or_else( || {
            panic!( "problem converting PathBuf obj to &str" );
        } );
        let marc_filepath: String = path_str.into();
        marc_filepaths.push( marc_filepath )
    }

    // -- clear output file
    fs::File::create( &output_filepath ).unwrap_or_else( |err| {
        panic!( "problem initializing the output file; error, ``{:?}``", err );
    });

    // -- get an append file-handler that i'll pass to the writer functions
    let fappend = fs::OpenOptions::new()
        .append( true )
        .open( &output_filepath )
        .unwrap_or_else( |err| {
            panic!( "problem initializing fappend; error, ``{:?}``", err );
        } );

    // loop through and process paths asynchronously
    let (tx, mut rx) = mpsc::channel( 100 );
    for marc_filepath in marc_filepaths {  // marc_filepath type-check yields: found struct `std::string::String`
        println!( "marc_filepath, ``{:?}``", marc_filepath );
        let mut tx = tx.clone();

        tokio::spawn( async move {
            let text_to_write: String = expensive_computation(
                &marc_filepath,
                &inner_title_field_tag,
                &inner_title_subfield_main_identifier,
                &inner_title_subfield_remainder_identifier,
                &inner_bib_field_tag,
                &inner_bib_subfield_bib_identifier,
                first_start_time
                ).await;
            tx.send( text_to_write ).await.unwrap_or_else( |err| {
                panic!( "problem sending on the transmit-end; error, ``{:?}``", err );
            } );
        } );

    }  // end for...

}  // end async fn main()...


herezz
async fn expensive_computation(
    marc_filepath: &str,
    inner_title_field_tag: &str,
    inner_title_subfield_main_identifier: &str,
    inner_title_subfield_remainder_identifier: &str,
    inner_bib_field_tag: &str,
    inner_bib_subfield_bib_identifier: &str,
    _first_start_time: time::Instant
    ) -> String {

    let _file_start_time = Instant::now();

    // -- load file into marc-reader
    let marc_records: Vec<marc::Record> = load_records( marc_filepath );

    // -- process records
    let mut text_to_write: String = "".to_string();
    let mut _counter: i32 = 0;
    for rec in marc_records.iter() {  // yields: `&marc::Record<'_>`
        let mut title: String = "".to_string();
        let mut bib: String = "".to_string();
        // println!("\nnew record...");
        for field in rec.field( Tag::from(inner_title_field_tag) ).iter() {
            // process_title( field, &title_subfield_main_identifier, &title_subfield_remainder_identifier, &output_filepath );
            title = process_title( field, inner_title_subfield_main_identifier, inner_title_subfield_remainder_identifier );
        }
        for field in rec.field( Tag::from(inner_bib_field_tag) ).iter() {
            // process_bib( field, &bib_subfield_bib_identifier, &output_filepath )
            bib = process_bib( field, inner_bib_subfield_bib_identifier );
        }

        text_to_write = format!( "{}\n{}\n\n{}", &title, &bib, text_to_write  );
        // text_to_write = format!( "title, ``{}``; bib, ``{}``", &title, &bib  );

        _counter += 1;
    }

    text_to_write
}

