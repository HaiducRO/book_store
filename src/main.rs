

mod models;
mod inventory;
mod sales;

use inventory::Inventory;
use models::Book;
use sales::Sales;

fn main() {
    let mut inventory = Inventory::new();
    if std::path::Path::new("inventory.json").exists() {
        let _ = inventory.load_invetory();
    } else {
        let _ = inventory.save_invetory();
    }
    
    loop {
        println!("--------------------------------");
        println!("Welcome to the Book Store!");
        println!("1. Add Book");
        println!("2. Remove Book");
        println!("3. List Books");
        println!("4. Sort Books");
        println!("5. Buy Book");
        println!("6. Edit Book");
        println!("7. Total Sales");
        println!("0. Exit");
        println!("--------------------------------");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                let mut title = String::new();
                let mut author = String::new();
                let mut genre = String::new();


                println!("Enter book title:");
                std::io::stdin().read_line(&mut title).unwrap();
                
                println!("Enter book author:");
                std::io::stdin().read_line(&mut author).unwrap();
                
                println!("Enter book genre:");
                std::io::stdin().read_line(&mut genre).unwrap();
                
                let price = read_valid_f32("Enter book price:");

                let quantity = read_valid_u32("Enter book quantity:");
                
                let book = Book::new(title.trim(), author.trim(), genre.trim(), price, quantity);
                inventory.add_book(book);
                let _ = inventory.save_invetory();
            }
            "2" => {
                let mut title = String::new();
                println!("Enter book title to remove:");
                std::io::stdin().read_line(&mut title).unwrap();
                inventory.remove_book(title.trim());
                let _ = inventory.save_invetory();
            }
            "3" => {
                inventory.list_books();
            }
            "4" => {
                println!("Sort by:");
                println!("1. Title");
                println!("2. Author");
                println!("3. Price");
                
                let mut sort_choice = String::new();
                std::io::stdin().read_line(&mut sort_choice).unwrap();

                match sort_choice.trim() {
                    "1" => inventory.sort_by_title(),
                    "2" => inventory.sort_by_author(),
                    "3" => inventory.sort_by_price(),
                    _ => println!("Invalid choice"),
                }
                inventory.list_books();
            }
            "5" => {
                let mut title = String::new();
                println!("Enter book title to buy:");
                std::io::stdin().read_line(&mut title).unwrap();


                let quantity: u32 = read_valid_u32("Enter quantity to buy:");
                
                inventory.sell_book(title.trim(), quantity);
                let _ = inventory.save_invetory();
            }
            "6" => {
                let mut title = String::new();
                println!("Enter book title to edit:");
                std::io::stdin().read_line(&mut title).unwrap();
                
                let mut new_title = String::new();
                let mut new_author = String::new();
                let mut new_genre = String::new();
                

                println!("Enter new book title (or leave blank to keep current):");
                std::io::stdin().read_line(&mut new_title).unwrap();
                
                println!("Enter new book author (or leave blank to keep current):");
                std::io::stdin().read_line(&mut new_author).unwrap();
                
                println!("Enter new book genre (or leave blank to keep current):");
                std::io::stdin().read_line(&mut new_genre).unwrap();
                
                let new_price = read_optional_f32("Enter new book price (or leave blank to keep current):");

                let new_quantity = read_optional_u32( "Enter new book quantity (or leave blank to keep current):");

                

                inventory.books.iter_mut()
                    .filter(|book| book.title == title.trim())
                    .for_each(|book| book.update_info(
                        if !new_title.trim().is_empty() { Some(new_title.trim()) } else { None },
                        if !new_author.trim().is_empty() { Some(new_author.trim()) } else { None },
                        if !new_genre.trim().is_empty() { Some(new_genre.trim()) } else { None },
                        if new_price.is_some() { Some(new_price.unwrap()) } else { None },
                        if new_quantity.is_some() { Some(new_quantity.unwrap()) } else { None },
                        
                    ));
                
                let _ = inventory.save_invetory();
            }

            "7" => {
                
                let total = Sales::total_sales();
                println!("Total sales: {:.2}", total);
                
            }
            
            "0" => break,
            _ => println!("Invalid choice"),
        }
    }
}


fn read_valid_f32(prompt: &str) -> f32 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<f32>() {
            Ok(value) => {
                if value < 0.0 {
                    println!("Value cannot be negative. Please enter a valid value.");
                    continue;
                }
                return value;
            }
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn read_valid_u32(prompt: &str) -> u32 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u32>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

pub fn read_optional_f32(prompt: &str) -> Option<f32> {
    use std::io::{self, Write};
    
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return None; 
        }

        match trimmed.parse::<f32>() {
            Ok(value) => return Some(value),
            Err(_) => {
                println!("Please enter a valid number.");
            }
        }
    }
}

pub fn read_optional_u32(prompt: &str) -> Option<u32> {
    use std::io::{self, Write};
    
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return None; 
        }

        match trimmed.parse::<u32>() {
            Ok(value) => return Some(value),
            Err(_) => {
                println!("Please enter a valid number.");
            }
        }
    }
}