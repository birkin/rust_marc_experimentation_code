// use marc::*;
use log::*;
use simple_logger::SimpleLogger;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use serde::Deserialize;
use serde_xml_rs::from_reader;
// use marc::Record;

use marc::{Field, Record, Subfield, Tag};


fn main() {

    // -- init logging
    // SimpleLogger::new().init().unwrap();  // or, to set the mininum level: ```SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();```
    SimpleLogger::new().with_level(log::LevelFilter::Debug).init().unwrap();
    
    // -- get marc file path
    // let marc_xml_path: String = "./source_files/sample_bibs_2022050222_7532401250006966_new_99.xml".to_string();
    let marc_xml_path: String = "./source_files/Incremental_set_wcollection_bibs_20230303031312.xml".to_string();
    debug!( "marc_xml_path, ``{:?}``", marc_xml_path);
        
    // -- load
    let marc_records: Vec<marc::Record> = load_records( &marc_xml_path );
    // debug!("first marc_record, ``{:?}``", marc_records[0]);
    // debug!("marc_records, ``{:?}``", marc_records);


    info!("end of main()");
}


fn load_records( marc_xml_path: &str ) -> Vec< marc::Record<'static> > {

    /*
        I believe the reason I need the `'static` 
     */

    // -- create the return Vec
    let mut result_vector: Vec<marc::Record> = Vec::new();

    // -- Read the MARC XML file
    // let file = File::open(marc_xml_path)?;
    let file = File::open(marc_xml_path).unwrap_or_else( |err| {
        panic!("could not open the marc_xml_path; error, ``{}``", err);
    });
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    // reader.read_to_string(&mut contents)?;
    reader.read_to_string(&mut contents).unwrap_or_else( |err| {
        panic!("could not read the file; error, ``{}``", err);
    });
    // debug!("contents, ``{:?}``", contents);

    // -- Deserialize the XML into a Collection
    // let collection: Collection = serde_xml_rs::from_str(&contents)?;
    let collection: Collection = serde_xml_rs::from_str(&contents).unwrap_or_else( |err| {
        panic!("could not deserialize the marc_xml; error, ``{}``", err);
    });
    // let zz: () = collection;
    debug!("collection, ``{:?}``", collection);

    return result_vector;
}

// -- load marc records from marc-xml file
// fn load_records( file_path: &str ) -> Vec< marc::Record<'static> > {

//     // create the return Vec
//     let mut result_vector: Vec<marc::Record> = Vec::new();

//     // create path-object to pass to file-handler
//     let path = Path::new( file_path );
//     let error_path_display = path.display();

//     // access the file
//     let file = match File::open(&path) {
//         Err(why) => panic!( "Couldn't open {}: {}", error_path_display, why.to_string() ),
//         Ok(file) => file,
//     };

//     // create a buffered reader
//     let reader = BufReader::new(file);

//     // iterate over the lines
//     for line in reader.lines() {
//         let line = line.unwrap();
//         let marc_record: Record = from_reader(line.as_bytes()).unwrap();
//         result_vector.push( marc_record );
//     }

//     // return the Vec
//     result_vector

// }  // end of load_records()


// -- Define structs to represent MARC XML structure
#[derive(Debug, Deserialize)]
struct Collection {
    #[serde(rename = "record", default)]
    records: Vec<RecordXml>,
}

#[derive(Debug, Deserialize)]
struct RecordXml {
    #[serde(rename = "datafield", default)]
    datafields: Vec<DataField>,
}

#[derive(Debug, Deserialize)]
struct DataField {
    #[serde(rename = "tag")]
    tag: String,
    #[serde(rename = "ind1")]
    ind1: String,
    #[serde(rename = "ind2")]
    ind2: String,
    #[serde(rename = "subfield", default)]
    subfields: Vec<SubField>,
}

#[derive(Debug, Deserialize)]
struct SubField {
    #[serde(rename = "code")]
    code: String,
    #[serde(rename = "$value")]
    // value: String,
    value: Option<String>,
}


// -- error syntax reminder
// let paths: glob::Paths = glob( &pattern ).unwrap_or_else( |err| {
//     panic!("could not glob the pattern; error, ``{}``", err);
// });
