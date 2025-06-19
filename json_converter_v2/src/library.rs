use crate::types::old_types::*;
use crate::types::new_types::*;
use std::collections::HashMap;
use uuid::Uuid;

impl Library {

    fn new() -> Self {
        Self {
            persons: HashMap::new(),
            items: vec!(),
            lookup: HashMap::new()
        }
    }
    pub fn convert_old_to_new_library(lib_old: &mut LibraryOld) -> Self {
        // getting items out of the old library
        let old_items = lib_old.items.clone();

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
                    let person = Self::director_to_person(director);
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
                ItemOld::Newspaper(newspaper_old) => {
                    let newspaper = Newspaper {
                        title: newspaper_old.title,
                        date: newspaper_old.date,
                    };
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

    fn check_if_person_exists(&mut self, person:&Person) -> Option<Uuid> {
        self.lookup.get(person).copied()
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

    fn add_item(&mut self, item:Item) -> &mut Self {
        self.items.push(item);
        self
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
    
    fn set_name(&mut self, new_name:String) -> &mut Self {
        self.name = new_name;
        self
    }
    
    pub fn get_name(&mut self) -> String {
        self.name.clone()
    }
    
    pub fn set_birth_year(&mut self, new_birth_year: u16) -> &mut Self {
        self.birth_year = Some(new_birth_year);
        self
    }
    
    pub fn get_birth_year(&mut self) -> Option<u16> {
        self.birth_year
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
    
    fn get_title(&mut self) -> String {
        self.title.clone()
    }
    
    fn get_year(&mut self) -> u16 {
        self.year
    }
    
    fn get_isbn(&mut self) -> String {
        self.isbn.clone()
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
    
    fn get_title(&mut self) -> String {
        self.title.clone()
    }
    
    fn get_year(&mut self) -> u16 {
        self.year
    }
    
    fn get_director(&mut self) -> Uuid {
        self.director
    }
    
}

impl Newspaper {
    fn new(title: String, date:String) -> Self {
        Self {title, date}
    }

    fn get_title(&mut self) -> String {
        self.title.clone()
    }

    fn get_date(&mut self) -> String {
        self.date.clone()
    }
}