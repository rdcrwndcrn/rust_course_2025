use crate::types::old_types::LibraryOld;
use crate::types::new_types::Library;
use std::fs::File;
use std::io::BufReader;
use serde_json::{from_reader, Error};

impl LibraryOld {
    pub fn read_library_from_json_file() -> Result<LibraryOld, Error> {
        let f = File::open("json_converter_v2/library.json");
        if f.is_ok(){
            // buffering that file - for efficiency purposes;
            // something with not directly using a Read instance
            let r = BufReader::new(f.unwrap());
            // using the serde_json::from_reader fn to deserialize our file
            let lib = from_reader(r);
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
            let lib = from_reader(r);
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