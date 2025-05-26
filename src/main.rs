use book_store::cli::{self, Choice};
use book_store::errs::ProjectResult;
use book_store::inventory::{self, Inventory};
use crate::cli::run_cli;
fn main() -> ProjectResult<()>{
    let mut inventory = Inventory::new();
        if std::path::Path::new("inventory.json").exists() {
            inventory.load_inventory()?;
        } else {
            inventory.save_inventory()?;
        }
    
        if let Err(e) = cli::run_cli(&mut inventory){
        eprintln!("Error: {}",e);
    }
    
    
    Ok(())
}
