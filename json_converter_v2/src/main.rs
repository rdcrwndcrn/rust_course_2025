mod types;
mod serde;
mod library;

use crate::types::old_types::LibraryOld;
use crate::types::new_types::Library;
use serde_json::to_string_pretty;


fn main() {
    let mut lib = LibraryOld::read_library_from_json_file().
        expect("Error: Cannot read lib json file");
    
    let s = to_string_pretty(&lib).unwrap();
    println!("Old lib: {s}");
    
    let mut new_lib = Library::convert_old_to_new_library(&mut lib);
    new_lib.sort_by_title();
    new_lib.write_library_to_json_file();

    let s = to_string_pretty(&new_lib).unwrap();
    println!("New lib: {s}");

}
