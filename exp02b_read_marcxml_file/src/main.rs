use marc::*;
use log::{debug, error, info, trace, warn};
use simple_logger::SimpleLogger;


fn main() {

    /// -- init logging
    SimpleLogger::new().init().unwrap();  // or, to set the mininum level: ```SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();```
    
    /// -- get marc file path
    let marc_path: String = "./source_files/sample_bibs_2022050222_7532401250006966_new_99.xml".to_string();
    debug!( "marc_path, ``{:?}``", marc_path);
        
    println!("Hello, world!");
}
