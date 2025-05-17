

mod models;
mod inventory;
mod sales;

use inventory::Inventory;
use models::Book;

fn main() {
    let mut inventory = Inventory::new();
    if std::path::Path::new("inventory.json").exists() {
        inventory.load_invetory();
    } else {
        inventory.save_invetory();
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
        println!("0. Exit");
        println!("--------------------------------");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                let mut title = String::new();
                let mut author = String::new();
                let mut genre = String::new();
                let mut price = String::new();
                let mut quantity = String::new();

                println!("Enter book title:");
                std::io::stdin().read_line(&mut title).unwrap();
                println!("Enter book author:");
                std::io::stdin().read_line(&mut author).unwrap();
                println!("Enter book genre:");
                std::io::stdin().read_line(&mut genre).unwrap();
                println!("Enter book price:");
                std::io::stdin().read_line(&mut price).unwrap();
                println!("Enter book quantity:");
                std::io::stdin().read_line(&mut quantity).unwrap();

                let price: f32 = price.trim().parse().unwrap();
                let quantity: u32 = quantity.trim().parse().unwrap();

                let book = Book::new(title.trim(), author.trim(), genre.trim(), price, quantity);
                inventory.add_book(book);
                inventory.save_invetory();
            }
            "2" => {
                let mut title = String::new();
                println!("Enter book title to remove:");
                std::io::stdin().read_line(&mut title).unwrap();
                inventory.remove_book(title.trim());
                inventory.save_invetory();
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
                
                let mut quantity = String::new();
                println!("Enter quantity to buy:");
                std::io::stdin().read_line(&mut quantity).unwrap();
                
                let quantity: u32 = quantity.trim().parse().unwrap();
                inventory.sell_book(title.trim(), quantity);
                inventory.save_invetory();
            }
            "6" => {
                let mut title = String::new();
                println!("Enter book title to edit:");
                std::io::stdin().read_line(&mut title).unwrap();
                
                let mut new_title = String::new();
                let mut new_author = String::new();
                let mut new_genre = String::new();
                let mut new_price = String::new();
                let mut new_quantity = String::new();

                println!("Enter new book title (or leave blank to keep current):");
                std::io::stdin().read_line(&mut new_title).unwrap();
                println!("Enter new book author (or leave blank to keep current):");
                std::io::stdin().read_line(&mut new_author).unwrap();
                println!("Enter new book genre (or leave blank to keep current):");
                std::io::stdin().read_line(&mut new_genre).unwrap();
                println!("Enter new book price (or leave blank to keep current):");
                std::io::stdin().read_line(&mut new_price).unwrap();
                println!("Enter new book quantity (or leave blank to keep current):");
                std::io::stdin().read_line(&mut new_quantity).unwrap();

                let price: Option<f32> = if !new_price.trim().is_empty() {
                    Some(new_price.trim().parse().unwrap())
                } else {
                    None
                };
                
                let quantity: Option<u32> = if !new_quantity.trim().is_empty() {
                    Some(new_quantity.trim().parse().unwrap())
                } else {
                    None
                };

                inventory.books.iter_mut()
                    .filter(|book| book.title == title.trim())
                    .for_each(|book| book.update_info(
                        if !new_title.trim().is_empty() { Some(new_title.trim()) } else { None },
                        if !new_author.trim().is_empty() { Some(new_author.trim()) } else { None },
                        if !new_genre.trim().is_empty() { Some(new_genre.trim()) } else { None },
                        price,
                        quantity,
                    ));
                
                inventory.save_invetory();
            }
            
            "0" => break,
            _ => println!("Invalid choice"),
        }
    }
}


