mod types;

use types::old_types::LibraryOld;
use types::new_types::Library;
use std::io::Error;
use std::fs::File;
use std::io::BufReader;

impl LibraryOld {
    fn read_library_from_json_file() -> Result<LibraryOld, Error> {
        let f = File::open("../library.json");
        if f.is_ok(){
            // buffering that file - for efficiency purposes;
            // something with not directly using a Read instance
            let r = BufReader::new(f.unwrap());
            // using the serde_json::from_reader fn to deserialize our file
            let lib: LibraryOld = from_reader(r);
            if lib.is_ok(){
                lib
            } else {
                panic!("Error: {:?}", lib.err());
            }
        } else {
            panic!("Error: {:?}", f.err());
        }
    }
}

impl Library {
    fn read_library_from_json_file() -> Result<Library, Error> {
        let f = File::open("../library_new.json");
        if f.is_ok(){
            let r = BufReader::new(f.unwrap());
            let lib:Library = from_reader(r);
            if lib.is_ok(){
                lib
            } else {
                panic!("Error: {:?}", lib.err());
            }
        } else {
            panic!("Error: {:?}", f.err());
        }
    }
}