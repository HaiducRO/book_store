use crossterm::{execute, terminal::{Clear, ClearType}};
use std::io::{stdout, Write};
use crate::errs::Error;
use crate::errs::ProjectResult;
use crate::inventory::Inventory;
use crate::models::Book;
use crate::sales::Sales;

pub enum Choice {
    list_books,
    sort_books,
    buy_book,
    add_book,
    remove_book,
    edit_book,
    total_sales,
}

impl Choice{
    
    pub fn execute(&self, inventory: &mut Inventory) -> ProjectResult<()>{
        
        
        match self {
            Choice::list_books => inventory.list_books(),
            
            Choice::sort_books => {
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
                    _ =>  println!("Invalid input"),
                }
                inventory.list_books()
            }

            Choice::buy_book => {
                let mut title = String::new();
                println!("Enter book title to buy:");
                std::io::stdin().read_line(&mut title).unwrap();
                let mut author = String::new();
                println!("Enter book author to buy:");
                std::io::stdin().read_line(&mut author).unwrap();
                
                let mut quantity = String::new();
                println!("Enter quantity to buy:");
                let _ = std::io::stdin().read_line(&mut quantity);
                let quantity: u32 = quantity.trim().parse::<u32>()?;

                inventory.sell_book(title.trim(),author.trim(),  quantity);
                println!("{} sold successfully!", title.trim());
                inventory.save_inventory()?;
            }

            Choice::add_book => {
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
                let _ = std::io::stdin().read_line(&mut price);
                let price: f32 = price.trim().parse::<f32>()?;


                println!("Enter book quantity:");
                let _ = std::io::stdin().read_line(&mut quantity);
                let quantity: u32 = match quantity.trim().parse::<u32>(){
                    Ok(val) => val,
                    Err(_) => return Err(Error::InvalidUnsigned(quantity.trim().to_string()))
                };
                
                let book = Book::new(title.trim(), author.trim(), genre.trim(), price, quantity);
                inventory.add_book(book);
                inventory.save_inventory()?;
            }

            Choice::remove_book => {   
                let mut title = String::new();
                println!("Enter book title to remove:");
                std::io::stdin().read_line(&mut title).unwrap();
                let mut author = String::new();
                println!("Enter book author to remove:");
                std::io::stdin().read_line(&mut author).unwrap();
                inventory.remove_book(title.trim(),author.trim());
                
                inventory.save_inventory()?;
            }

            Choice::edit_book => {
                let mut title = String::new();
                println!("Enter book title to edit:");
                std::io::stdin().read_line(&mut title).unwrap();
                let mut author = String::new();
                println!("Enter book author to edit:");
                std::io::stdin().read_line(&mut author).unwrap();
                inventory.remove_book(title.trim(),author.trim());
                
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
                let _ = std::io::stdin().read_line(&mut new_price);
                let new_price: f32 = new_price.trim().parse::<f32>()?;

                println!("Enter new book quantity (or leave blank to keep current):");
                let _ = std::io::stdin().read_line(&mut new_quantity);
                let new_quantity: u32 = new_quantity.trim().parse::<u32>()?;
                

                inventory.books.iter_mut()
                    .filter(|book| book.title == title.trim() || book.author == author.trim())
                    .for_each(|book| book.update_info(
                        if !new_title.trim().is_empty() { Some(new_title.trim()) } else { None },
                        if !new_author.trim().is_empty() { Some(new_author.trim()) } else { None },
                        if !new_genre.trim().is_empty() { Some(new_genre.trim()) } else { None },
                        if !new_price.is_nan() { Some(new_price) } else { None },
                        if new_quantity != 0 { Some(new_quantity) } else { None },
                        
                    ));
                inventory.save_inventory()?;
            }

            Choice::total_sales => {
                let total = Sales::total_sales();
                println!("Total sales: {:.2}", total);
            }
        }
        Ok(())
    }

    
}

pub fn run_cli(mut inventory: &mut Inventory) -> ProjectResult<()>{
        
        loop{
            println!("\n===== Bookstore Menu =====");
            println!("1. List books");
            println!("2. Sort books");
            println!("3. Buy book");
            println!("4. Add book");
            println!("5. Remove book");
            println!("6. Edit book");
            println!("7. View total sales");
            println!("0. Exit");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            match input.trim() {
                "1" => {
                    Choice::list_books.execute(&mut inventory)?;
                },
                "2" => {
                    Choice::sort_books.execute(&mut inventory)?;
                    clear_console()?;
                },
                "3" => {
                    Choice::buy_book.execute(&mut inventory)?;
                    clear_console()?;
                },
                "4" => {
                    Choice::add_book.execute(&mut inventory)?;
                    clear_console()?;
                },
                "5" => {
                    Choice::remove_book.execute(&mut inventory)?;
                    clear_console()?;
                },
                "6" => {
                    Choice::edit_book.execute(&mut inventory)?;
                    clear_console()?;
                },
                "7" => Choice::total_sales.execute(&mut inventory)?,
                "0" => {
                    inventory.save_inventory()?;
                    println!("Exiting...");
                    clear_console()?;
                    break;
                }
                
                _ => {
                    clear_console()?;
                    println!("Invalid choice. Try again.");
                },
            
            }
            
        }
    Ok(())
        
}

pub fn clear_console() -> ProjectResult<()>{
    execute!(stdout(), Clear(ClearType::All)).expect("something went wrong");
    Ok(())
}