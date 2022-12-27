use reqwest::*;
use serde_json;
use tokio::*;

#[tokio::main]
async fn main() {
    // let search_url: String = "https://repository.library.brown.edu/api/search/?q=&selected_facets=mods_type_of_resource%3Amoving+image".to_string();
    let search_url: String = "http://httpbin.org/json".to_string();

    let resp = reqwest::get(search_url).await.expect("REASON").text().await;
    println!("body = {:?}", resp);
}

// http://httpbin.org/json
