use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub price: f32,
    pub quantity: u32,
}

impl Book {
    pub fn new(title: &str, author: &str, genre: &str, price: f32, quantity: u32) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            genre: genre.to_string(),
            price,
            quantity,
        }
    }

    pub fn update_info(&mut self, title: Option<&str>, author: Option<&str>, genre: Option<&str>, price: Option<f32>, quantity: Option<u32>) {
        if let Some(t) = title {
            self.title = t.to_string();
        }
        if let Some(a) = author {
            self.author = a.to_string();
        }
        if let Some(g) = genre {
            self.genre = g.to_string();
        }
        if let Some(p) = price {
            self.price = p;
        }
        if let Some(q) = quantity {
            self.quantity = q;
        }
    }

}