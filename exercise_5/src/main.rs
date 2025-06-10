#![allow(dead_code)]

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

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
enum ItemOld {
    BookOld(BookOld),
    NewspaperOld(NewspaperOld),
    MovieOld(MovieOld)
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BookOld {
    title: String,
    year: u16,
    author: Author
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct AuthorOld {
    name: String,
    birth_year: u16
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MovieOld {
    title: String,
    year: u16
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Newspaper {
    title: String,
    date: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Library {
    persons: HashMap<Uuid, Person>,
    items: Vec<Item>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
enum Item {
    Book(Book),
    Movie(Movie),
    Newspaper(Newspaper)
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Person {
    id: Uuid,
    name: String,
    birth_year: u16
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Book {
    title: String,
    year: u16,
    author: Vec<Uuid>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Movie {
    title: String,
    year: u16,
    director: Vec<Uuid>
}

impl LibraryOld {

    fn read_library_file() -> Result<LibraryOld, Error> {
        // opens our file binding it to f
        let f = File::open("rust_course_2025/exercise_5/library.json");
        // check if the result is ok or an error
        if f.is_ok(){
            // buffering that file - for efficiency purposes,
            // something with not directly using a Read instance
            let r = BufReader::new(f.unwrap());
            // using the serde_json::from_reader fn to deserializing our file
            // into the existing data structure: Library
            let lib = from_reader(r);
            // check if the result of operation is ok or error
            if lib.is_ok(){
                lib
            } else {
                panic!("Error: {:?}", lib.err());
            }
        } else {
            panic!("Error: {:?}", f.err());
        }
    }

    fn write_library_file(&mut self) {
        // opens our file binding it to f
        let mut f = match File::open("rust_course_2025/exercise_5/library.json"){
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

    fn add_book(&mut self, title:String, year:u16, author: AuthorOld) -> &mut Self {
        self.add_item(ItemOld::BookOld(BookOld::new(title, year, author)));
        self
    }

    fn add_newspaper(&mut self, title:String, date:String) -> &mut Self {
        self.add_item(ItemOld::Newspaper(Newspaper::new(title, date)));
        self
    }

    fn add_movie(&mut self, title:String, year:u16) -> &mut Self {
        self.add_item(ItemOld::MoviOlde(MovieOld::new(title, year)));
        self
    }

    fn add_series(&mut self, series: Vec<ItemOld>) -> &mut Self {
        self.items.extend(series);
        self
    }

    fn add_book_series(&mut self, author: AuthorOld) -> &mut Self {
        let j = get_u16_input("the number of books in the series");
        for _ in 1..=j {
            let book = BookOld::new(
                    get_string_input(
                        format!("the name of the {}. book", j).as_str()),
                    get_u16_input(
                        format!("the year the {}. book was written", j).as_str()),
                    author.clone());
            let i = ItemOld::BookOld(book);
            self.add_item(i);
        }
        self
    }
}

impl Book {
    fn new(title:String, year:u16, author: Author) -> Self {
        Self {title, year, author}
    }

    fn set_title(&mut self, new_title:String) -> &mut Self{
        self.title = new_title;
        self
    }

    fn set_year(&mut self, new_year:u16) -> &mut Self {
        self.year = new_year;
        self
    }

    fn set_author(&mut self, new_author:Author) -> &mut Self {
        self.author = new_author;
        self
    }
}

impl Movie {
    fn new(title:String, year:u16) -> Self {
        Self{title, year}
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
            items: vec!()
        }
    }

    fn read_library_file() -> Result<Library, Error> {
        let f = File::open("rust_course_2025/exercise_4/library.json")?;
        let r = BufReader::new(f.unwrap());
        let lib = from_reader(f)?;
        Ok(lib)
    }

    fn write_library_file(&mut self) {
        let mut f = File::open("rust_course_2025/exercise_4/library.json")?;
        serde_json::to_writer_pretty(&mut f, self)?;
    }

    fn add_item(&mut self, item:Item) -> &mut Self {
        self.items.push(item);
    }

    fn add_person(&mut self, person:Person) -> &mut Self {
        self.persons.insert(Uuid::new_v4(), person);
    }

    fn add_book(&mut self, title:String, year:u16, author: Author) -> &mut Self {
        self.add_item(Item::Book(Book::new(title, year, author)));
        self
    }

    fn add_newspaper(&mut self, title:String, date:String) -> &mut Self {
        self.add_item(Item::Newspaper(Newspaper::new(title, date)));
        self
    }

    fn add_movie(&mut self, title:String, year:u16) -> &mut Self {
        self.add_item(Item::Movie(Movie::new(title, year)));
        self
    }

    fn add_series(&mut self, series: Vec<Item>) -> &mut Self {
        self.items.extend(series);
        self
    }
}

impl Person {
    fn new(name:String, birth_year:u16) -> Self{
        Person{
            id: Uuid::new_v4(),
            name,
            birth_year
        }
    }
}

impl Book {
    fn new(title: String, year: u16, author: Vec<Uuid>) -> Self {
        Self {
            title,
            year,
            author
        }
    }
}

impl Movie {
    fn new(title: String, year: u16, director: Vec<Uuid>) -> Self {
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
    // reading hard coded library.json, and binding it with match
    let l = match Library::read_library_file() {
        Ok(lib) => lib,
        Err(e) => panic!("Error: {:?}", e),
    };

    // pretty printing our library
    // using serde_json's pretty print
    let s = to_string_pretty(&l).unwrap();
    println!("Pretty print: {s}");
}

/*
    - Platz in Privatsprechstunde?
    - Problem: Schmerzen im unteren Rücken; vielleicht wegen leichter Wirbelsäulenverkrümmung
    - ... und
    <<<<15:30 9.7, Handtuch 
 */