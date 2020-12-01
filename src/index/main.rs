extern crate dotenv;
extern crate meilisearch_sdk;
use dotenv::var;
use meilisearch_sdk::client::*;

#[tokio::main]
async fn main() {
    println!("Creating index!");
    let meili_addr = var("MEILISEARCH_HOST").unwrap();
    let client = Client::new(&meili_addr, "master");
    let food = client
        .get_or_create("food")
        .await
        .expect("Failed to init index!");

    println!("{:#?}", food);
}
