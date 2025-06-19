mod types;

use std::fs::File;
use types::old_types::LibraryOld;
use types::new_types::Library;
impl LibraryOld {
    fn write_library_to_json_file(&mut self) {
        // opens our file binding it to f
        let mut f = match File::create("rust_course_2025/json_converter_v1/library.json"){
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
    fn write_library_to_json_file(&mut self) {
        // if its existing crate deletes it, what's in it, otherwise just creating the file
        let mut f = File::create("../library_new.json")
            // so we don't need to match the error with match
            // expect does the same, Ok(value) => values
            // and when there is an Err(), it displays our custom error message
            .expect("Error: Something with opening the file we want to write in");
        serde_json::to_writer_pretty(&mut f, self)
            .expect("Error: Who even needs pretty writing.. right?");
    }
}