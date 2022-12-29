// use std::arch::aarch64::int32x2_t;
use std::collections::HashMap;

// use reqwest::*;
use serde_json;
// use tokio::*;

#[tokio::main]
async fn main() {
    // load settings ----------------------------
    let settings: HashMap<String, String> = load_settings().await;
    println!("settings, ``{:?}``", settings);

    let test = settings.get("TRACKER_PATH").unwrap().clone();

    // extract search-url from settings hashmap
    // let search_url: &str = &settings["SEARCH_URL"];
    // let search_url: String = (&settings["SEARCH_URL"]).to_string();

    // let initial_search_url: &str = &settings["SEARCH_URL"];
    // let search_url: String = initial_search_url.to_string();

    // query search-url -------------------------
    // let search_url: String = settings.get("SEARCH_URL").unwrap().clone();
    let search_url: String = settings
        .get("SEARCH_URL")
        .expect("problem accessing SEARCH_URL setting")
        .clone();
    let search_result: String = get_search_json(search_url).await;

    // let search_result: String = get_search_json(       // <--- this works
    //     settings.get("SEARCH_URL").unwrap().clone()
    // ).await;

    //convert to json-object using serde_json
    let search_json: serde_json::Value =
        serde_json::from_str(&search_result).unwrap_or_else(|error| {
            panic!(
                "Problem converting search_json to json-object: ``{:?}``",
                error
            );
        });
}

async fn load_settings() -> HashMap<String, String> {
    /* Loads settings from envars */
    let mut envar_settings: HashMap<String, String> = HashMap::new();
    let tracker_path: String =
        std::env::var("TRNSCRER_PRPPR__TRACKER_PATH").unwrap_or_else(|error| {
            panic!( "Problem accessing environmental-variable ``TRNSCRER_PRPPR__TRACKER_PATH``: ``{:?}``", error );
        });
    envar_settings.insert("TRACKER_PATH".to_string(), tracker_path);
    // let search_url: String = "https://repository.library.brown.edu/api/search/?q=&selected_facets=mods_type_of_resource%3Amoving+image".to_string();
    let search_url: String = "http://httpbin.org/json".to_string();
    envar_settings.insert("SEARCH_URL".to_string(), search_url);
    envar_settings
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
