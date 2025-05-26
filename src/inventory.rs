

use serde::{Deserialize, Serialize};
use uuid::Uuid;


use crate::errs::ProjectResult;
use crate::models::Book;
use crate::sales::Sales;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Inventory {
    pub books: Vec<Book>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            books: Vec::new(),
        }
    }

    // Load inventory from inventory.json
    pub fn load_inventory(&mut self) -> ProjectResult<()> {
        let file =  std::fs::File::open("inventory.json")?;
        let reader = std::io::BufReader::new(file);
        let inventory: Inventory = serde_json::from_reader(reader)?;
        self.books = inventory.books;
        Ok(())
   
    }
    // Save inventory to inventory.json
    pub fn save_inventory(&self) -> ProjectResult<()> {
        let file =  std::fs::File::create("inventory.json")?;
        let writer = std::io::BufWriter::new(file);   
        serde_json::to_writer_pretty(writer, &self)?;
        Ok(())        
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
    pub fn remove_book(&mut self, title: &str, author: &str) {
        self.books.retain(|book| book.title != title && book.author != author);
    }

    pub fn list_books(&self) {
        for book in &self.books {
            println!("Title: {}, Author: {}, Genre: {}, Price: ${}, Quantity: {}", book.title, book.author, book.genre, book.price ,book.quantity);
        }
    }

    pub fn sort_by_title(&mut self) {
        self.books.sort_by(|a, b| a.title.cmp(&b.title));
    }

    pub fn sort_by_author(&mut self) {
        self.books.sort_by(|a, b| a.author.cmp(&b.author));
    }

    pub fn sort_by_price(&mut self) {
        self.books.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
    }

    pub fn update_info(&mut self, title: &str, author: &str, new_title: Option<&str>, new_author: Option<&str>, new_genre: Option<&str>, new_price: Option<f32>, new_quantity: Option<u32>) {
        if let Some(book) = self.books.iter_mut().find(|b| b.title == title && b.author == author) {
            book.update_info(new_title, new_author, new_genre, new_price, new_quantity);
        } else {
            println!("Book not found");
        }
    }

    pub fn sell_book(&mut self, book_title: &str,book_author: &str, quantity: u32) {
        if let Some(book) = self.books.iter_mut().find(|b| b.title == book_title && b.author == book_author) {
            if book.quantity >= quantity {
                book.quantity -= quantity;
                let total_price = book.price * quantity as f32;
                let sale = Sales::new(Uuid::new_v4(), &book.title, quantity, total_price);
                let _ = sale.sale_log();
            } else {
                println!("Not enough stock for {}", book_title);
            }
        } else {
            println!("Book not found");
        }
    }
}




