// use reqwest::*;
// use tokio::*;
use log::{debug, LevelFilter}; // for logging
use serde_json;
use std::collections::HashMap;
use std::io::Write; // for logging

#[tokio::main]
async fn main() {
    // setup logging ----------------------------
    build_logger().await;
    debug!("hello world");

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

async fn build_logger() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] {} [{}::{}] {}",
                // chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                chrono::Local::now().format("%d/%b/%Y %H:%M:%S"),
                record.level(),
                record.file().unwrap_or("unknown"),
                // record.target(),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .filter(Some("the_logger"), LevelFilter::Debug)
        .init();
    debug!("logging started");
}

async fn load_settings() -> HashMap<String, String> {
    /* Loads settings from envars */
    let mut envar_settings: HashMap<String, String> = HashMap::new();
    // tracker-path -----------------------------
    let envar_key: String = "TRNSCRBR_META_PRPPR__TRACKER_PATH".to_string();
    let tracker_path: String = std::env::var(&envar_key).unwrap_or_else(|error| {
        panic!(
            "Problem accessing envar, ``{:?}``; error, ``{:?}``",
            envar_key, error
        );
    });
    envar_settings.insert("TRACKER_PATH".to_string(), tracker_path);
    // search-url -------------------------------
    let envar_key: String = "TRNSCRBR_META_PRPPR__INITIAL_SEARCH_URL".to_string();
    let search_url: String = std::env::var(&envar_key).unwrap_or_else(|error| {
        panic!(
            "Problem accessing envar, ``{:?}``; error, ``{:?}``",
            envar_key, error
        );
    });
    envar_settings.insert("SEARCH_URL".to_string(), search_url);
    debug!(
        "envar_settings formatted, ``{}``",
        serde_json::to_string_pretty(&envar_settings).unwrap()
    );
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
