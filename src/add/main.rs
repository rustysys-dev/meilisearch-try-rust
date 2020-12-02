extern crate dotenv;
extern crate meilisearch_sdk;
extern crate serde;
use dotenv::var;
use meilisearch_sdk::{client::*, document::*, progress::*};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::prelude::*};

#[derive(Serialize, Deserialize, Debug)]
struct Food {
    id: i64,
    item: String,
    genre: Vec<String>,
}

impl Document for Food {
    type UIDType = i64;

    fn get_uid(&self) -> &Self::UIDType {
        &self.id
    }
}

#[tokio::main]
async fn main() {
    println!("Creating index!");
    let meili_addr = var("MEILISEARCH_HOST").unwrap();
    let client = Client::new(&meili_addr, "master");
    let food = client
        .get_or_create("food")
        .await
        .expect("Failed to init index!");

    let mut file = File::open("data.json/docs.json").expect("failed to open file!");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Could not read from file");

    let food_docs: Vec<Food> = serde_json::from_str(&content).expect("failed to deserialize!");
    let result = food
        .add_documents(&food_docs, None)
        .await
        .expect("failed to add documents!");

    let status: ProcessedStatus;
    loop {
        let result_status = result.get_status().await.expect("failed to get status.");
        match result_status {
            Status::Processed(s) => {
                status = s;
                break;
            }
            Status::Enqueued(_) => (),
        }
    }

    println!("{:#?}", status);
}
