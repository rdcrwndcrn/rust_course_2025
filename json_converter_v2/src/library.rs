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
        let old_items = lib_old.items.clone();
        let mut lib = Library::new();
        for item_variant in old_items {
            // destructing items and adding the newly structured objects to the new library
            match item_variant {
                ItemOld::Book(BookOld{ title, year, isbn, authors }) => {
                    let mut persons: Vec<Uuid> = Vec::new();
                    for author in authors {
                        // make person from author
                        let person = Self::author_to_person(author);
                        let mut uuid = Uuid::new_v4();
                        // check if the person is already in lib person before adding it
                        let exists = lib.person_lookup(&person);
                        if exists.is_some(){
                            // person already in HashMap
                            uuid = exists.unwrap();
                        } else {
                            // person not in HashMap
                            lib.add_person(person, uuid);
                        }
                        // add uuid to vec for the authors-field of this book
                        persons.push(uuid);

                    }
                    let authors = persons;
                    lib.add_item(Item::Book(Book{title, year, isbn, authors}));
                }
                ItemOld::Movie(MovieOld{ title, year, director }) => {
                    // make person from director
                    let person = Self::director_to_person(director);
                    let mut director_uuid= Uuid::new_v4();
                    // check if the director already exists in lib persons
                    let exists = lib.person_lookup(&person);
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

    fn person_lookup(&mut self, person:&Person) -> Option<Uuid> {
        self.lookup.get(person).copied()
    }

    fn add_person(&mut self, person:Person, uuid: Uuid) -> &mut Self {
        // to be safe to check if already in the library
        let exists = self.person_lookup(&person);
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

    fn filter_book(&self) -> Vec<&Book> {
        self.items
            .iter()
            .filter(|item| matches!(item, Item::Book(_)))
            .map(|item| if let Item::Book(b) = item {
                b
            } else {
                unreachable!()
            })
            .collect()
    }

    fn filter_newspaper(&mut self) -> Vec<&Newspaper> {
        self.items
            .iter()
            .filter_map(|x| match x {
                Item::Newspaper(n) => Some(n),
                _ => None
            })
            .collect()
    }

     fn filter_movie(&mut self) -> Vec<&Movie> {
         self.items
             .iter()
             .filter_map(|m| match m {
                 Item::Movie(m) => Some(m),
                 _ => None
             })
             .collect()
     }

    fn search_by_title(&mut self, search: &str) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|item| match item {
                Item::Book(b)  => b.title.contains(search),
                Item::Movie(m )  => m.title.contains(search),
                Item::Newspaper(n) => n.title.contains(search)
            })
            .collect()
    }

    // sorts items by title
    pub fn sort_by_title(&mut self) {
       self.items.sort_by_key(|item| match item {
               Item::Movie(m) => m.title.clone(),
               Item::Newspaper(n) => n.title.clone(),
               Item::Book(b) => b.title.clone()
           });
    }

    /* sorts persons by name
    fn sort_by_name(&mut self) {
        let mut persons_vec: Vec<(Uuid, Person)> = self.persons.drain().collect(); 
        persons_vec.sort_by_key(|(_,person)| person.name.clone());
        for (u, p) in persons_vec {
            self.persons.insert(u, p);
        }
    }
     */

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

    fn get_name(&mut self) -> String {
        self.name.clone()
    }

    fn set_birth_year(&mut self, new_birth_year: u16) -> &mut Self {
        self.birth_year = Some(new_birth_year);
        self
    }

    fn get_birth_year(&mut self) -> Option<u16> {
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