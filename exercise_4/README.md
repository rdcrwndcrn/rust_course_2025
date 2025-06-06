# Exercise 4: Enums and Collections #1

## Preparation

In this set of tasks, we will learn about a second interesting enum and the basics of parsing JSON with `serde`,

### Enum Result

Sometimes using an `Option` is not enough. With `Option`, we only can tell if there is some data or nothing.
But how about handling errors? For this, Rust gives us the enum `Result<T, E>`:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E)
}
```

In short, if there is no error, `Ok(T)` is returned holding our data (similar to `Some(T)`), otherwise `Err(E)` with the
error message.
For more details, please read
the [relevant chapter](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html).

### Parsing JSON with Serde

When it comes to parsing files in Rust, `serde` (**Ser**ializing and **De**serializing) is the way to go. This framework
offers parser for many formats, e.g. XML, YAML, JSON.
Here, we'll use the `serde_json` extension.

What we need to know is, that our structs/enums must derive `Deserialize` in order to be used by `serde`.
If we want to write JSON content, we need to derive `Serialize`, too. \
Besides the derive, `serde` comes with its own set of makro definitions. Commonly used is the definition
for `CamelCase`:

```rust
// #[serde(rename_all = "camelCase")]
// ...
```

Please have a look at the [documentation](https://serde.rs).

## Tasks

### 1) Rustlings

Work yourself through the exercise sets 05, 08 and 12.
You can run Rustlings with Gitpod, or just install the whole package on your local system.

### 2) Combining Struct and Enum

Analyze the given library database "library.json".

- Extend your code base from the _struct exercise 03_
- Use an Enum to represent the new types besides `Book`

**Help/Hints**

For the library, the following struct can be used:

```rust
struct Library {
    // Again, we use a Vector here - this time typed (our type 'Item', or your own definition)
    items: Vec<Item>,
}
```

#### A) Read and Parse files using "serde"

With this task we dive into the depths of file parsing and processing using the external crate `serde` (pre-configured).

- Read about the basics of working with `serde`
- Annotate your structs and enums properly (e.g. using the [examples](https://serde.rs/examples.html))
- Implement a function that reads the library file into a buffer and returns the content parsed by `serde`
   - For opening and buffered reading of files, see [here](https://doc.rust-lang.org/std/fs/struct.File.html)
   - Use the function `from_reader` offered by `serde_json`
- At the end, pretty-print the library

**Help/Hints**

- To handle `Result` properly, `match` should be used
- Aborting the program on error can be done with makro `panic!()`

#### B) Extending the implementation: New items

- Implement functions/methods for adding new items (books, newspaper, movie)
   - Re-use and update your implementation from the struct exercise
- What needs to be done to implement the CRUD pattern? [_**Advanced**_, Theoretical]
   - CRUD: Create | Read | Update | Delete

#### C) Saving states back to file

Update your implementation so, that the library is written back to file before the program exits.

- We need to derive `Serialize` for our types
   - Again, check the serde examples provided on the website

### 3) Additional Tasks

Work yourself through the following tutorial page and solve the tasks given.

* https://practice.course.rs/compound-types/enum.html
* https://practice.course.rs/collections/string.html
* https://practice.course.rs/collections/vector.html
