#![allow(dead_code)]

use std::collections::HashMap;
use std::io;
use std::fs::{File};
use std::io::{BufReader};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_string_pretty, Error};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
struct LibraryOld {
    items: Vec<ItemOld>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
enum ItemOld {
    Book(BookOld),
    Newspaper(Newspaper),
    Movie(MovieOld)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct BookOld {
    title: String,
    year: u16,
    // newly added, for this JSON type
    isbn: String,
    // changed to plural
    authors: Vec<AuthorOld>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct AuthorOld {
    name: String,
    birth_year: Option<u16>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct DirectorOld {
    name: String,
    birth_year: Option<u16>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct MovieOld {
    title: String,
    year: u16,
    director: DirectorOld
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Newspaper {
    title: String,
    date: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Library {
    persons: HashMap<Uuid, Person>,
    items: Vec<Item>,
    #[serde(skip)]
    lookup: HashMap<Person, Uuid>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
enum Item {
    Book(Book),
    Movie(Movie),
    Newspaper(Newspaper)
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
struct Person {
    name: String,
    birth_year: Option<u16>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Book {
    title: String,
    year: u16,
    isbn: String,
    authors: Vec<Uuid>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Movie {
    title: String,
    year: u16,
    director: Uuid
}

impl LibraryOld {

    fn read_library_from_json_file() -> Result<LibraryOld, Error> {
        // opens our file binding it to f
        let f = File::open("../library.json");
        // check if the result is ok or an error
        if f.is_ok(){
            // buffering that file - for efficiency purposes,
            // something with not directly using a Read instance
            let r = BufReader::new(f.unwrap());
            // using the serde_json::from_reader fn to deserializing our file
            // into the existing data structure: Library
            let lib = from_reader(r);
            // check if the result of the operation is ok or error
            if lib.is_ok(){
                lib
            } else {
                panic!("Error: {:?}", lib.err());
            }
        } else {
            panic!("Error: {:?}", f.err());
        }
    }

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

    fn add_item(&mut self, i: ItemOld) -> &mut Self {
        self.items.push(i);
        self
    }

    fn add_newspaper(&mut self, title:String, date:String) -> &mut Self {
        self.add_item(ItemOld::Newspaper(Newspaper::new(title, date)));
        self
    }

    fn add_series(&mut self, series: Vec<ItemOld>) -> &mut Self {
        self.items.extend(series);
        self
    }

    fn get_items(&mut self) -> Vec<ItemOld> {
        self.items.clone()
    }
}

impl BookOld {
    fn new(title:String, year:u16, isbn:String, authors: Vec<AuthorOld>) -> Self {
        Self {title, year, isbn, authors}
    }

    fn set_title(&mut self, new_title:String) -> &mut Self{
        self.title = new_title;
        self
    }

    fn set_year(&mut self, new_year:u16) -> &mut Self {
        self.year = new_year;
        self
    }
}

impl MovieOld {
    fn new(title:String, year:u16, director:DirectorOld) -> Self {
        Self{title, year, director}
    }

    fn set_title(&mut self, new_title:String) -> &mut Self {
        self.title = new_title;
        self
    }

    fn set_year(&mut self, new_year:u16) -> &mut Self {
        self.year = new_year;
        self
    }
}

impl Newspaper {
    fn new(title: String, date:String) -> Self {
        Self {title, date}
    }

    fn set_title(&mut self, new_title:String) -> &mut Self {
        self.title = new_title;
        self
    }

    fn set_date(&mut self, new_date:String) -> &mut Self {
        self.date = new_date;
        self
    }
}

impl Library {
    fn new() -> Self {
        Self {
            persons: HashMap::new(),
            items: vec!(),
            lookup: HashMap::new()
        }
    }
    
    fn write_library_to_json_file(self) {
        // if its existing crate deletes it, what's in it, otherwise just creating the file
        let mut f = File::create("../library_new.json")
            // so we don't need to match the error with match
            // expect does the same, Ok(value) => values,
            // if it works, and if not, it displays our custom error message
            .expect("Error: Something with opening the file we want to write in");
        serde_json::to_writer_pretty(&mut f, &self)
            .expect("Error: Who even needs pretty writing.. right?");
    }

    fn read_from_library_old(lib_old: &mut LibraryOld) -> Self {
        // getting items out of the old library
        let old_items = lib_old.get_items();

        // initializing a new lib
        let mut lib = Library::new();

        // iterating over the items
        for item_variant in old_items {
            // destructing the item and adding the newly structured objects to the new library
            match item_variant {
                ItemOld::Book(BookOld{ title, year, isbn, authors }) => {
                    // vector with persons uuid for book authors
                    let mut persons: Vec<Uuid> = Vec::new();
                    // looping over the authors
                    for author in authors {
                        // creating persons from authors
                        let person = Self::author_to_person(author);
                        let mut uuid = Uuid::new_v4();
                        // check if the person is already in lib person before adding it
                        let exists = lib.check_if_person_exists(&person);
                        if exists.is_some(){
                            // person already in HashMap
                            uuid = exists.unwrap();
                            persons.push(uuid);
                        } else {
                            // person not in HashMap
                            // adds person to lib 
                            lib.add_person(person, uuid);
                            // adds uuid of the author to persons
                            persons.push(uuid);
                        }
                        
                    }
                    let authors = persons;
                    // creating a book object and adding it to the library
                    lib.add_item(Item::Book(Book{title, year, isbn, authors}));
                }
                ItemOld::Movie(MovieOld{ title, year, director }) => {
                    // creating person from director
                    let person = Library::director_to_person(director);
                    let mut director_uuid= Uuid::new_v4();
                    // check if the director already exists in lib persons
                    let exists = lib.check_if_person_exists(&person);
                    if exists.is_some(){
                        // person already exists 
                        director_uuid = exists.unwrap();
                    } else {
                        // person does not exist
                        // adding person to lib
                        lib.add_person(person.clone(), director_uuid);
                    }
                    // creating and adding movie to lib
                    lib.add_item(Item::Movie(Movie{title, year, director: director_uuid}));
                }
                ItemOld::Newspaper(newspaper) => {
                    // the struct did not change
                    // adding Newspaper to lib
                    lib.add_item(Item::Newspaper(newspaper));
                }
            }
        }
        lib

    }

    // making a person from an authors struct; for libraryOld -> libraryNew
    fn author_to_person(AuthorOld { name, birth_year }: AuthorOld) -> Person {
        Person {
            name,
            birth_year
        }
    }

    // making a person from a director struct; for libraryOld -> libraryNew
    fn director_to_person(DirectorOld{ name, birth_year }: DirectorOld) -> Person {
        Person {
            name,
            birth_year
        }
    }

    fn add_item(&mut self, item:Item) -> &mut Self {
        self.items.push(item);
        self
    }

    fn add_person(&mut self, person:Person, uuid: Uuid) -> &mut Self {
        // to be safe to check if already in the library
        let exists = self.check_if_person_exists(&person);
        // if true person does already exist
        if exists.is_some(){
            return self;
        }
        // person does not exist
        // add to persons and lookup
        self.persons.insert(uuid, person.clone());
        self.lookup.insert(person, uuid);
        self
    }
    
    fn check_if_person_exists(&mut self, person:&Person) -> Option<Uuid> {
        self.lookup.get(person).copied()
    }

    fn add_book(&mut self, title:String, year:u16, isbn:String, author: Vec<Uuid>) -> &mut Self {
        self.add_item(Item::Book(Book::new(title, year, isbn, author)));
        self
    }

    fn add_newspaper(&mut self, title:String, date:String) -> &mut Self {
        self.add_item(Item::Newspaper(Newspaper::new(title, date)));
        self
    }

    fn add_movie(&mut self, title:String, year:u16, director: Uuid) -> &mut Self {
        self.add_item(Item::Movie(Movie::new(title, year, director)));
        self
    }

    fn add_series(&mut self, series: Vec<Item>) -> &mut Self {
        self.items.extend(series);
        self
    }
}

impl Person {
    fn new(name:String, birth_year:Option<u16>) -> Self{
        Person{
            name,
            birth_year
        }
    }
}

impl Book {
    fn new(title: String, year: u16, isbn:String, authors: Vec<Uuid>) -> Self {
        Self {
            title,
            year,
            isbn,
            authors
        }
    }
}

impl Movie {
    fn new(title: String, year: u16, director: Uuid) -> Self {
        Movie {
            title,
            year,
            director
        }
    }
}

fn get_string_input(message: &str) -> String {
    println!("Please put in {} and press enter: ", message);
    let mut input = String::new();
    let read_result = io::stdin().read_line(&mut input);
    match read_result {
        Ok(_) => input.trim().to_string(),
        Err(_) => get_string_input(message)
    }
}

fn get_u16_input(message: &str) -> u16 {
    println!("Please put in {} and press enter: ", message);
    let mut input = String::new();
    let _read_result = io::stdin().read_line(&mut input);
    match input.trim().parse::<u16>() {
        Ok(n) => n,
        Err(_) => get_u16_input(message)
    }
}

fn main() {
    // reading hard-coded library.json and binding it with a match
    let mut l = LibraryOld::read_library_from_json_file().
        expect("Error: Cannot read lib json file");

    // using serde_json's pretty print the new lip
    let s = to_string_pretty(&l).unwrap();
    println!("Pretty print: {s}");
    
    // converting the data structure to the new one
    let new_l = Library::read_from_library_old(&mut l);
    
    // writing the new one to a file
    new_l.write_library_to_json_file();
}