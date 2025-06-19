/* Rust by example - 3.1 - destructing
// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn class_rect_area(x: f32, y: f32) -> f64 {
    x as f64 * y as f64
}

fn point_rect_area(p: Point) -> f64 {
    p.x as f64 * p.y as f64
}

fn rect_area(r: &Rectangle) -> f64 {
    let Rectangle {
        top_left:     Point { x: x1, .. },
        bottom_right: Point { x: x2, y: y2 },
    } = *r;
    x1 as f64 * y2 as f64
}

impl Rectangle {
    fn area(&self) -> f32 {
       self.top_left.x * self.bottom_right.y
    }
}


fn square(p: Point, right:f32) -> Rectangle {
    Rectangle {
        top_left: p,
        bottom_right: Point {x:0f32, y:right},
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 10.3, ..another_point };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
*/


use std::io;

fn get_string_input(message: &str) -> String {
    println!("Please put in {} and press enter: ", message);
    let mut input = String::new();
    let read_result = io::stdin().read_line(&mut input);
    match read_result {
        Ok(_) => input.trim().to_string(),
        Err(_) => get_string_input(message)
    }
}

fn get_i16_input(message: &str) -> i16 {
    println!("Please put in {} and press enter: ", message);
    let mut input = String::new();
    let _read_result = io::stdin().read_line(&mut input);
    match input.trim().parse::<i16>() {
        Ok(n) => n,
        Err(_) => get_i16_input(message)
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
// Task 2 - Structs
struct Author {
    name: String,
    year: i16
}

impl Author {
    fn new (name:String, year: i16) -> Self {
        Self {
            name,
            year
        }
    }

    fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn set_year(&mut self, new_year:i16) {
        self.year = new_year;
    }
}

#[derive(Debug,PartialEq, Eq)]
struct Book{
    name: String,
    year: i16,
    author: Author
}

impl Book {
    fn new(name:String, year:i16, author: Author) -> Self {
        Self {
            name,
            year,
            author
        }
    }

    fn set_name(&mut self, new_name:String){
        self.name = new_name;
    }

    fn set_year(&mut self, new_year:i16){
        self.year = new_year;
    }

    fn set_author(&mut self, new_author:Author){
        self.author = new_author;
    }

    fn add_series(author: Author) -> Vec<Book> {
        let mut series: Vec<Book> = Vec::new();
        let mut i = get_i16_input("the number of books in the series");
        let mut j = 1;
        loop {
            let book = Book::new(get_string_input(format!("the Name of the {}. Book", j).as_str()),
                                 get_i16_input(format!("the year the {}. Book was written", j).as_str()),
                                 author.clone());
            series.push(book);
            i -= 1;
            j += 1;
            if i == 0 {
                break;
            }
        }
        series
    }
}




fn main() {
    // adding the Robert Langdon series
    let author = Author::new(String::from("Dan Brown"), 1964);
    let series = Book::add_series(author);
    println!("The whole series of Robert Langdon: {:#?}", series);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let author = Author {
            name:String::from("Dan Brown"),
            year:1964
        };
        let other_author = Author::new(String::from("Dan Brown"), 1964);
        assert_eq!(author, other_author);

        let book = Book{
            name:String::from("The Da Vinci Code"),
            year:2003,
            author
        };
        let other_book = Book::new(String::from("The Da Vinci Code"), 2003, other_author);
        assert_eq!(book, other_book);
    }

    #[test]
    fn setter(){
        let mut author = Author::new(String::from("Dan Brown"), 1964);
        let other_author = Author::new(String::from("Luca Pacioli"), 1445);

        author.set_name(String::from("Sebastian Fitzek"));
        assert_eq!(author.name, "Sebastian Fitzek");

        author.set_year(1971);
        assert_eq!(author.year, 1971);

        let mut book = Book::new(String::from("The Da Vinci Code"), 2003, author);

        book.set_name(String::from("De Divina Proportione"));
        assert_eq!(book.name, String::from("De Divina Proportione"));

        book.set_year(1498);
        assert_eq!(book.year, 1498);

        book.set_author(Author::new(String::from("Luca Pacioli"), 1445));
        assert_eq!(book.author, other_author);
    }
}