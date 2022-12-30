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

    // get facet-query result -------------------
    let search_result: String = get_search_json(settings["SEARCH_URL"].to_string()).await;

    //convert to json-object using serde_json
    let search_json: serde_json::Value =
        serde_json::from_str(&search_result).unwrap_or_else(|error| {
            panic!(
                "Problem converting search_json to json-object: ``{:?}``",
                error
            );
        });
    println!(
        "search_json formattd, ``{}``",
        serde_json::to_string_pretty(&search_json).unwrap()
    );
} // end main()

async fn load_settings() -> HashMap<String, String> {
    /* Loads settings from envars */
    let mut envar_settings: HashMap<String, String> = HashMap::new();
    let tracker_path: String =
        std::env::var("TRNSCRBR_PRPPR__TRACKER_DIR_PATH").unwrap_or_else(|error| {
            panic!("Problem accessing environmental-variable: ``{:?}``", error);
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
