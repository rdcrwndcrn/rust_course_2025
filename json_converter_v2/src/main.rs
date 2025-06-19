mod types;
mod serde;
mod library;

use crate::types::old_types::LibraryOld;
use crate::types::new_types::Library;
use serde_json::to_string_pretty;


fn main() {
    let mut l = LibraryOld::read_library_from_json_file().
        expect("Error: Cannot read lib json file");
    
    let s = to_string_pretty(&l).unwrap();
    println!("Pretty print: {s}");
    
    let new_l = Library::convert_old_to_new_library(&mut l);

    new_l.write_library_to_json_file();
}
