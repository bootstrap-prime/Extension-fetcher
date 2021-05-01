use serde::{Deserialize, Serialize};
use std::fs;
use std::{collections::HashMap, env};

static DEFAULT_FILENAME: &'static str = "./extensions.toml";

#[derive(Serialize, Deserialize)]
pub struct Extensions {
    extensions: HashMap<String, String>,
}

pub struct TomlList {
    human_name: String,
    api_slug: String,
}

pub struct ExtensionData {
    name: String,
    url: String,
    hash: String,
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

    let toml_ext_data: Extensions = match toml::from_str(&file_contents) {
        Ok(data) => data,
        Err(e) => {
            println!("{}", e);
            std::process::exit(-1);
        }
    };
    // let toml_ext_data = toml_ext_data.extensions;

    let responses = match get_mozilla_api_responses(toml_ext_data.extensions) {
        Ok(data) => data,
        Err(e) => {
            println!("{}", e);
            std::process::exit(-1);
        },
    };

    for element in responses {
        println!("(pkgs.fetchFirefoxAddon {{");
        println!("  name = \"{}\";          ", element.name);
        println!("  url = \"{}\";           ", element.url);
        println!("  sha256 = \"{}\";        ", element.hash);
        println!("}})                       ");
    }

    //get_firefox_url(&tomltable);
}

fn toml_data_to_struct(tomldata: HashMap<String, String>) -> Vec<TomlList> {
    let mut structured_data: Vec<TomlList> = Vec::with_capacity(tomldata.len());
    for (key, value) in tomldata {
        structured_data.push(
            TomlList {
                human_name: key.to_string(),
                api_slug: value.to_string(),
            }
        );
    }

    return structured_data;
}

fn get_mozilla_api_up() {
    // check to see if site is functional
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

fn get_mozilla_api_responses(exts: HashMap<String, String> ) -> Result<Vec<ExtensionData>, reqwest::Error> {
    let api = "https://addons.mozilla.org/api/v4/addons/addon";
    let client = reqwest::blocking::Client::new();
    let mut gathered_extensions: Vec<ExtensionData> = Vec::with_capacity(exts.len());

    // check to see if site is functional
    // addons.mozilla.org/api/v4/site (check)
    get_mozilla_api_up();

    for (human_name, api_slug) in exts.iter() {
        let client = &client;
        let request_url: String = format!("{}/{}/", api, api_slug);

        let resp = client.get(request_url).send()?.json::<Root>()?;

        gathered_extensions.push(ExtensionData {
            name: human_name.to_string(),
            url: resp.current_version.files[0].url.to_string(),
            hash: resp.current_version.files[0].hash.to_string(),
        });
    }

    return Ok(gathered_extensions);
}
