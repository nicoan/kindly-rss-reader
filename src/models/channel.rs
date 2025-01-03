use serde::Serialize;

#[derive(Serialize)]
pub struct Channel {
    pub title: String,
    pub items: Vec<Item>,
}

#[derive(Serialize)]
pub struct Item {
    pub guid: String,
    pub title: String,
    pub categories: Vec<String>,
    pub date: String,
    pub description: String,
    pub author: String,
}
