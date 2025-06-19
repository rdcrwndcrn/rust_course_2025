use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) mod old_types {
    use super::*;
    #[derive(Deserialize, Serialize, Debug)]
    pub struct LibraryOld {
        pub items: Vec<ItemOld>
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(tag = "type", rename_all = "camelCase")]
    pub enum ItemOld {
        Book(BookOld),
        Newspaper(NewspaperOld),
        Movie(MovieOld)
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct BookOld {
        pub title: String,
        pub year: u16,
        pub isbn: String,
        pub authors: Vec<AuthorOld>
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct AuthorOld {
        pub name: String,
        pub birth_year: Option<u16>
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct DirectorOld {
        pub name: String,
        pub birth_year: Option<u16>
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct MovieOld {
        pub title: String,
        pub year: u16,
        pub director: DirectorOld
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct NewspaperOld {
        pub title: String,
        pub date: String,
    }
}

pub (crate) mod new_types {
    use super::*;
    #[derive(Deserialize, Serialize, Debug)]
    pub struct Library {
        pub persons: HashMap<Uuid, Person>,
        pub items: Vec<Item>,
        #[serde(skip)]
        pub lookup: HashMap<Person, Uuid>
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(tag = "type", rename_all = "camelCase")]
    pub enum Item {
        Book(Book),
        Movie(Movie),
        Newspaper(Newspaper)
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
    #[serde(rename_all = "camelCase")]
    pub struct Person {
        pub name: String,
        pub birth_year: Option<u16>
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Book {
        pub title: String,
        pub year: u16,
        pub isbn: String,
        pub authors: Vec<Uuid>
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Movie {
        pub title: String,
        pub year: u16,
        pub director: Uuid
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Newspaper {
        pub title: String,
        pub date: String,
    }
}