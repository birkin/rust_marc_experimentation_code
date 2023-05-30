use log::*;
use rayon::prelude::*;
use serde::Deserialize;
use simple_logger::SimpleLogger;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    // -- init logging
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    // -- set marc file path
    // let marc_xml_path: String =
    //     "./source_files/Incremental_set_wcollection_bibs_20230303031312.xml".to_string(); // 42 records

    let marc_xml_path: String =
        "./source_files/sample_bibs_2022050222_7532401250006966_new_99.xml".to_string(); // 10,000 records
    debug!("marc_xml_path, ``{:?}``", marc_xml_path);

    // -- load xml
    let marc_records: Collection = load_records(&marc_xml_path);
    // debug!("first marc_record, ``{:?}``", marc_records.records[0]);

    // -- iterate through records
    // for record in marc_records.records.iter() {  // original syntax
    //     process_record(record);
    // }
    marc_records.records.par_iter().for_each(|record| {
        process_record(record);
    });

    info!("end of main()");
}

fn process_record(record: &RecordXml) {
    let title: String = parse_title(&record);
    let author: String = parse_author(&record);
    let alma_mmsid: String = parse_alma_mmsid(&record);
    let bibnum: String = parse_bibnum(&record);
    println!(
        "title, ``{:?}``; author, ``{:?}``; alma_mmsid, ``{:?}``; bibnum, ``{:?}``",
        title, author, alma_mmsid, bibnum
    );
}

fn parse_bibnum(record: &RecordXml) -> String {
    let mut bibnum = String::new();
    for datafield in &record.datafields {
        if datafield.tag == "907" {
            for subfield in &datafield.subfields {
                if subfield.code == "a" {
                    bibnum = subfield.value.clone().unwrap_or_else(|| "".to_string());
                }
            }
        }
    }
    bibnum
}

fn parse_alma_mmsid(record: &RecordXml) -> String {
    let mut alma_mmsid = String::new();
    for datafield in &record.datafields {
        if datafield.tag == "001" {
            for subfield in &datafield.subfields {
                if subfield.code == "a" {
                    alma_mmsid = subfield.value.clone().unwrap_or_else(|| "".to_string());
                }
            }
        }
    }
    alma_mmsid
}

fn parse_title(record: &RecordXml) -> String {
    let mut title = String::new();
    for datafield in &record.datafields {
        if datafield.tag == "245" {
            for subfield in &datafield.subfields {
                if subfield.code == "a" {
                    title = subfield.value.clone().unwrap_or_else(|| "".to_string());
                    // title explanation: <https://gist.github.com/birkin/57952fa4052167ddb8b5c98ec8beb920>
                }
            }
        }
    }
    title
}

fn parse_author(record: &RecordXml) -> String {
    let mut author = String::new();
    for datafield in &record.datafields {
        if datafield.tag == "100" {
            for subfield in &datafield.subfields {
                if subfield.code == "a" {
                    author = subfield.value.clone().unwrap_or_else(|| "".to_string());
                }
            }
        }
    }
    author
}

fn load_records(marc_xml_path: &str) -> Collection {
    // -- Read the MARC XML file
    // let file = File::open(marc_xml_path)?;
    let file = File::open(marc_xml_path).unwrap_or_else(|err| {
        panic!("could not open the marc_xml_path; error, ``{}``", err);
    });
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    // reader.read_to_string(&mut contents)?;
    reader.read_to_string(&mut contents).unwrap_or_else(|err| {
        panic!("could not read the file; error, ``{}``", err);
    });
    // debug!("contents, ``{:?}``", contents);

    // -- Deserialize the XML into a Collection
    let collection: Collection = serde_xml_rs::from_str(&contents).unwrap_or_else(|err| {
        panic!("could not deserialize the marc_xml; error, ``{}``", err);
    });

    // -- log the collection
    let collection_str = format!("{:?}", collection);
    let collection_substr_ellipses =
        format!("{}...", &collection_str[..collection_str.len().min(200)]);
    debug!("collection (partial), ``{:?}``", collection_substr_ellipses);

    return collection;
}

// ------------------------------------------------------------------
// -- Simple-item struct --------------------------------------------

// struct Item {
//     alma_mmsid: String,
//     bibnum: String,
//     title: String,
//     author: String,
// }

// ------------------------------------------------------------------

// -- Structs to represent MARC XML structure -----------------------

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

// ------------------------------------------------------------------
// -- Scratch work --------------------------------------------------

// -- error syntax reminder
// let paths: glob::Paths = glob( &pattern ).unwrap_or_else( |err| {
//     panic!("could not glob the pattern; error, ``{}``", err);
// });

// let zz: () = collection;

// -- build the marc::Record objects
// for record_xml in collection.records {
//     let mut marc_record = marc::Record::new();
//     for datafield in record_xml.datafields {
//         let mut marc_field = marc::Field::new(datafield.tag.as_str());
//         marc_field.set_indicator1(datafield.ind1.as_str());
//         marc_field.set_indicator2(datafield.ind2.as_str());
//         for subfield in datafield.subfields {
//             marc_field.add_subfield(marc::Subfield::new(
//                 subfield.code.as_str(),
//                 subfield.value.unwrap_or_else(|| "".to_string()).as_str(),
//             ));
//         }
//         marc_record.append(marc_field);
//     }
//     result_vector.push(marc_record);
// }
