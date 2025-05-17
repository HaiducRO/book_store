use chrono::DateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::io::Write;
use std::io::BufRead;


#[derive(Serialize, Deserialize, Debug)]
pub struct Sales {
    pub id: Uuid,
    pub book_title: String,
    pub quantity_sold: u32,
    pub total_price: f32,
    pub timestamp: DateTime<chrono::Utc>,
}

impl Sales{
    pub fn new(id: Uuid, book_title: &str, quantity_sold: u32, total_price: f32) -> Sales {
        Sales {
            id,
            book_title: book_title.to_string(),
            quantity_sold,
            total_price,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn sale_log(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("sales_log.jsonl")?;

        let json = serde_json::to_string(self)?;
        writeln!(file, "{}", json)?; 

        Ok(())
    }

    pub fn total_sales() -> f32 {
        let mut total = 0.0;

        if let Ok(file) = std::fs::File::open("sales_log.jsonl") {
            let reader = std::io::BufReader::new(file);

            for line in reader.lines() {
                if let Ok(json) = line {
                  if let Ok(sale) = serde_json::from_str::<Sales>(&json) {
                       total += sale.total_price;
                 }
                }
            }
        } else {
            println!("sales_log.jsonl not found or couldn't be opened.");
        }

            total
        }   
}