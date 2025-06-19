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
    pub fn lib_converter(lib_old: &mut LibraryOld) -> Self {
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

    fn check_if_person_exists(&mut self, person:&Person) -> Option<Uuid> {
        self.lookup.get(person).copied()
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
}