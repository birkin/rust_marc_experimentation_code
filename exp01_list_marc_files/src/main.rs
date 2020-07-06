extern crate glob;  // <https://docs.rs/glob/0.3.0/glob/>

// use std::fmt::Debug;
// use std::fmt::Display;

use glob::glob;


fn main() {

    let file_download_dir: String = "./source_files".to_string();
    let pattern: String = format!( "{}/*.mrc", file_download_dir );
    println!("pattern, ``{:?}``", pattern);

    let paths = glob( &pattern ).unwrap_or_else( |err| {
        panic!("could not glob the pattern; error, ``{}``", err);
    });
    // let zz: () = paths;  // yields: expected `()`, found struct `glob::Paths`
    // println!("paths, ``{:?}``", paths);  // error: `glob::Paths` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
    // println!("paths, ``{}``", paths);  // error: `glob::Paths` cannot be formatted with the default formatter

    let iter: Vec<glob::GlobResult> = paths.into_iter().collect();  // <https://docs.rs/glob/0.3.0/glob/type.GlobResult.html>
    // let zz: () = iter;  // yields: expected `()`, found struct `std::vec::Vec`
    println!( "iter, ``{:?}``", iter );
}


// -- Output...

// pattern, ``"../../source_files/*.mrc"``
// iter, ``[Ok("../../source_files/sierra_export_0726.mrc")]``
