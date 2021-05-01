use serde::{Deserialize, Serialize};
use std::fs;
use std::{collections::HashMap, env};

static DEFAULT_FILENAME: &'static str = "./extensions.toml";

#[derive(Serialize, Deserialize)]
pub struct Extensions {
    extensions: HashMap<String, String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &str;
    if args.len() >= 2 {
        filename = &args[1];
    } else {
        filename = DEFAULT_FILENAME;
    }

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => {
            println!("{}", e);
            std::process::exit(-1);
        }
    };

    let toml_ext_data: Extensions = toml::from_str(&file_contents).unwrap();

    //get_firefox_url(&tomltable);
}

async fn get_firefox_url(exts: &Extensions) {
    let mut nix_collected = HashMap::new();

    let api = "https://addons.mozilla.org/api/v4/addons/addon";

    let client = reqwest::Client::new();

    // check to see if site is functional
    // addons.mozilla.org/api/v4/site (check)

    exts.extensions.values().for_each(|xtension| {
        nix_collected.insert("name", xtension);
        let request_url_base = format!("{}/{}/", api, xtension);

        // parse api response from slug
        let unparsed_response = reqwest::get(request_url).await?.text();
        let parsed_response = serde_json::from_str(&unparsed_response).unwrap();

        //let content = parsed_response.

    });

    for(name, xtname) in &nix_collected {
        println!("{}: \"{}\"", name, xtname);
    }
}

// serde things for deserializing api response for values we care about
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "current_version")]
    //#[serde(flatten)]
    pub current_version: CurrentVersion,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentVersion {
    pub files: Vec<File>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub hash: String,
    pub url: String,
}

// name comes from deserialized toml, api_response comes from mozilla addon api
fn parse_mozilla_api_response(api_response: &str, name: &str) -> HashMap<String, String> {
    let mut extension_data = HashMap::new();
    extension_data.insert("name".to_string(), name.to_string());

    let parsed_response: Root = match serde_json::from_str(api_response) {
        Ok(value) => value,
        Err(e) => {
            println!("{}", e);
            std::process::exit(-1);
        }
    };

    extension_data.insert(
        "url".to_string(),
        parsed_response.current_version.files[0].url.to_string(),
    );
    extension_data.insert(
        "hash".to_string(),
        parsed_response.current_version.files[0].hash.to_string(),
    );

    for (key, value) in &extension_data {
        println!("{} = {}", key, value);
    }

    return extension_data;
}
