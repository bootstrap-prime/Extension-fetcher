use serde::{Serialize, Deserialize};
use std::{collections::HashMap, env};
use std::fs;

static DEFAULT_FILENAME: &'static str = "./extensions.toml";

#[derive(Serialize, Deserialize)]
pub struct Extensions {
    extensions: HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "current_version")]
    pub current_version: CurrentVersion,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentVersion {
    pub files: Vec<File>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub hash: String,
    pub url: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename:&str;
    if args.len() >= 2 {
        filename = &args[1];
    } else {
        filename = DEFAULT_FILENAME;
    }

    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => {
            contents
        }
        Err(e) => {
            println!("{}", e);
            std::process::exit(-1);
        }
    };

    let tomltable: Extensions = toml::from_str(&file_contents).unwrap();

    get_firefox_url(&tomltable);
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
