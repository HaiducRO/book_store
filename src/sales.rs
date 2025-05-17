use chrono::DateTime;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sales {
    pub id: i32,
    pub book_title: String,
    pub quantity_sold: u32,
    pub total_price: f32,
    pub timestamp: DateTime<chrono::Utc>,
}

impl Sales{
    pub fn new(id: i32, book_title: &str, quantity_sold: u32, total_price: f32) -> Sales {
        Sales {
            id,
            book_title: book_title.to_string(),
            quantity_sold,
            total_price,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn sale_log(&self) {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("sales.jsonl")
            .unwrap();
        let writer = std::io::BufWriter::new(file);
        serde_json::to_writer(writer, self).unwrap();
    }
}