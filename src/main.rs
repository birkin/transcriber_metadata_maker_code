use std::arch::aarch64::int32x2_t;

use reqwest::*;
use serde_json;
use tokio::*;

#[tokio::main]
async fn main() {
    // let search_url: String = "https://repository.library.brown.edu/api/search/?q=&selected_facets=mods_type_of_resource%3Amoving+image".to_string();
    let search_url: String = "http://httpbin.org/json".to_string();
    println!("search_url, ``{}``", search_url);

    // let resp = reqwest::get(search_url).await.expect("REASON").text().await;
    let search_json: String = get_search_json(search_url).await;
    // println!("body = {:?}", search_json);
}

async fn get_search_json(search_url: String) -> String {
    println!("starting get_search_json()");

    // query search-url -------------------------
    let resp = reqwest::get(search_url).await.unwrap_or_else(|error| {
        panic!("Problem accessing search-url: ``{:?}``", error);
    });
    println!("resp, ``{:?}``", resp);

    // get text from response -------------------
    let search_json: String = resp.text().await.unwrap_or_else(|error| {
        panic!("Problem extracting text from response: ``{:?}``", error);
    });
    println!("search_json, ``{:?}``", search_json);

    // return -----------------------------------
    search_json
}

// misc stuff -------------------------------------------------------

// http://httpbin.org/json
