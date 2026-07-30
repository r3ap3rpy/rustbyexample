#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}
fn borrow_book(book: &Book) {
    println!("Book borrowed: {} - {}",book.title, book.year);
}
fn new_edition(book: &mut Book) {
   book.year = 2026;
   println!("New edition of book: {} - {}",book.title, book.year);
}
fn main() {
    let immutable_book = Book {
        author: "Daniel Szabo",
        title: "Rust programming",
        year: 2000,
    };
    let mut mutable_book = immutable_book;
    borrow_book(&immutable_book);
    borrow_book(&mutable_book);
    new_edition(&mut mutable_book);
}
