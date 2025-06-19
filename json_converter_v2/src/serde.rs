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

    fn write_library_to_json_file(&self) {
        // opens our file binding it to f
        let mut f = match File::create("json_converter_v2/library.json"){
            Ok(file) => file,
            Err(e) => panic!("Error: {e}"),
        };
        // serializing the library to JSON and into the file
        match serde_json::to_writer_pretty(&mut f, self){
            Ok(s) => s,
            Err(e) => panic!("Error: {e}"),
        };
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

    pub fn write_library_to_json_file(&self) {
        // if its existing crate deletes it, what's in it, otherwise just creating the file
        let mut f = File::create("json_converter_v2/library_new.json")
            // so we don't need to match the error with match
            // expect does the same, Ok(value) => values
            // and when there is an Err(), it displays our custom error message
            .expect("Error: Something with opening the file we want to write in");
        serde_json::to_writer_pretty(&mut f, self)
            .expect("Error: Who even needs pretty writing.. right?");
    }
}