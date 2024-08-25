use meilisearch_sdk::client::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SampleData {
    id: usize,
    title: String,
    family_name: Vec<String>,
}

#[tokio::main]
async fn main() {
    let client = Client::new(
        "http://127.0.0.1:7700",
        Some("1b85758befd0e2d3fd70f898966856af681c36848194addab18b21b6b180d34c"),
    )
    .unwrap();

    //set document name
    // An index is where the documents are stored.
    let sample_index = client.index("sample");

    //add index data
    // Add some movies in the index. If the index 'movies' does not exist, Meilisearch creates it when you first add the documents.
    sample_index
        .add_documents(
            &[
                SampleData {
                    id: 1,
                    title: "Hello".to_string(),
                    family_name: vec![
                        "Alice".to_string(),
                        "Bob".to_string(),
                        "Charlie".to_string(),
                    ],
                },
                SampleData {
                    id: 2,
                    title: "Hello".to_string(),
                    family_name: vec!["Henly".to_string(), "Nyaki".to_string(), "Cacy".to_string()],
                },
                SampleData {
                    id: 3,
                    title: "Hello".to_string(),
                    family_name: vec!["Yamada".to_string(), "Bob".to_string(), "Kevin".to_string()],
                },
            ],
            Some("id"),
        )
        .await
        .unwrap();
}
