use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiJsonRoot {
    pub contents: Vec<Content>,
}
