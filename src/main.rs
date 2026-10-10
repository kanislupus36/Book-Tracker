
use std::fs;
use std::io::{self, Write};

// Represents a single book in the collection.
struct Book {
    title: String,
    author: String,
    is_read: bool,
}

impl Book {
    // Creates a new book that has not been read yet.
    fn new(title: String, author: String) -> Book {
        Book {
            title,
            author,
            is_read: false,
        }
    }

    // Borrows the book to display its information.
    fn display(&self, number: usize) {
        let status = if self.is_read { "Read" } else { "Unread" };

        println!(
            "{}. {} by {} [{}]",
            number, self.title, self.author, status
        );
    }
}

// Gets text input from the user.
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to display prompt");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

// Adds a new book to the collection.
fn add_book(books: &mut Vec<Book>) {
    let title = get_input("Enter book title: ");
    let author = get_input("Enter author: ");

    if title.is_empty() || author.is_empty() {
        println!("Title and author cannot be empty.");
        return;
    }

    // The book is moved into the collection.
    books.push(Book::new(title, author));

    println!("Book added successfully!");
}

// Displays all books in the collection.
fn view_books(books: &[Book]) {
    if books.is_empty() {
        println!("Your book collection is empty.");
        return;
    }

    println!("\n=== Your Books ===");

    // Borrow each book instead of taking ownership.
    for (index, book) in books.iter().enumerate() {
        book.display(index + 1);
    }
}

// Marks a selected book as read.
fn mark_book_read(books: &mut [Book]) {
    if books.is_empty() {
        println!("There are no books to update.");
        return;
    }

    view_books(books);

    let input = get_input("Enter the book number to mark as read: ");

    let number: usize = match input.parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    if number == 0 || number > books.len() {
        println!("That book number does not exist.");
        return;
    }

    // Subtract one because displayed numbers start at 1.
    let index = number - 1;

    if books[index].is_read {
        println!("That book is already marked as read.");
    } else {
        books[index].is_read = true;
        println!("Book marked as read!");
    }
}

// Searches for books by title.
fn search_books(books: &[Book]) {
    let query = get_input("Enter a title to search for: ");
    let query_lower = query.to_lowercase();
    let mut found = false;

    for (index, book) in books.iter().enumerate() {
        if book.title.to_lowercase().contains(&query_lower) {
            book.display(index + 1);
            found = true;
        }
    }

    if !found {
        println!("No matching books found.");
    }
}

// Saves the collection to a text file.
fn save_books(books: &[Book]) {
    let mut contents = String::new();

    for book in books {
        // Each line stores the title, author, and read status.
        contents.push_str(&format!(
            "{}|{}|{}\n",
            book.title, book.author, book.is_read
        ));
    }

    match fs::write("books.txt", contents) {
        Ok(_) => println!("Books saved to books.txt!"),
        Err(error) => println!("Failed to save books: {}", error),
    }
}

// Loads the collection from a text file.
fn load_books() -> Vec<Book> {
    let contents = match fs::read_to_string("books.txt") {
        Ok(contents) => contents,
        Err(_) => {
            println!("No saved collection found. Starting a new one.");
            return Vec::new();
        }
    };

    let mut books = Vec::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.splitn(3, '|').collect();

        if parts.len() == 3 {
            let title = parts[0].to_string();
            let author = parts[1].to_string();
            let is_read = parts[2].parse::<bool>().unwrap_or(false);

            books.push(Book {
                title,
                author,
                is_read,
            });
        }
    }

    println!("Loaded {} book(s).", books.len());
    books
}

fn main() {
    // Load previously saved books when the program starts.
    let mut books = load_books();

    // Keep showing the menu until the user chooses to exit.
    loop {
        println!("\n=== Book Tracker ===");
        println!("1. Add Book");
        println!("2. View Books");
        println!("3. Mark Book as Read");
        println!("4. Search Books");
        println!("5. Save Books");
        println!("6. Load Books");
        println!("7. Exit");

        let choice = get_input("Choose an option: ");

        match choice.as_str() {
            "1" => add_book(&mut books),
            "2" => view_books(&books),
            "3" => mark_book_read(&mut books[..]),
            "4" => search_books(&books),
            "5" => save_books(&books),
            "6" => books = load_books(),
            "7" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please select 1-7."),
        }
    }
}
