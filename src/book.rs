#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub borrowed: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct User {
    pub username: String,
    pub loaned_books: Vec<Book>,
}

impl Book {
    pub fn new(title: String, author: String, isbn: String, borrowed: bool) -> Book {
        Book{
            title,
            author,
            isbn,
            borrowed,
        }
    }

    pub fn print_book(&self){
        println!("Book: ");
        println!("-----------");
        println!("{}", self.title);
        println!("{}", self.author);
        println!("{}", self.isbn);
        println!("-----------");
        println!()
    }

    pub fn borrow(&mut self){
        self.borrowed = true;
    }

    pub fn return_book(&mut self){
        self.borrowed = false;
    }
}

impl User {
    pub fn new(username: String, loaned_books: Vec<Book>) -> User {
        User {
            username,
            loaned_books,
        }
    }

    pub fn print_user(&self) {
        println!("{:?}", self);
    }

    pub fn add_book(&mut self, book: &mut Book) {
        if !book.borrowed {
            book.borrowed = true;
            self.loaned_books.push(book.clone());
        }
    }

    pub fn remove_return_book(&mut self) {
        if !self.loaned_books.is_empty() {
            // For demonstration, simply pop one book and mark it as returned.
            let mut book = self.loaned_books.pop().unwrap();
            book.borrowed = false;
        }
    }
}


pub fn fill_book_data(mut lib: Vec<Book>) -> Vec<Book> {

    let temp = vec!{
        Book{
            title: "test".into(),
            author: "Mr. Tester".into(),
            isbn: "00000800".into(),
            borrowed: false,
        },
        Book{
            title: "test2".into(),
            author: "Mr. Tester2".into(),
            isbn: "00000802".into(),
            borrowed: false,
        },
        Book{
            title: "test3".into(),
            author: "Mr. Tester3".into(),
            isbn: "00000803".into(),
            borrowed: false,
        },
        Book {
            title: "test4".into(),
            author: "Mr. Tester4".into(),
            isbn: "00000804".into(),
            borrowed: false,
        }
    };

    for book in temp {
        lib.push(book);
    };

    lib
}

pub fn fill_user_data(mut member: Vec<User>) -> Vec<User> {
    let temp = vec!{
        User{
            username: "John".into(),
            loaned_books: vec![]
        },
        User{
            username: "Bob".into(),
            loaned_books: vec![]
        },
        User{
            username: "Will".into(),
            loaned_books: vec![]
        },
        User{
            username: "Della".into(),
            loaned_books: vec![]
        }
    };

    for user in temp {
        member.push(user);
    };

    member
}