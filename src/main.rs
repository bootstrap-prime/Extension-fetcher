use serde::Deserialize;
use std::fs;
use std::{collections::HashMap, env};

static DEFAULT_FILENAME: &'static str = "./extensions.toml";

#[derive(Deserialize)]
pub struct Extensions {
    extensions: HashMap<String, String>,
}

fn main() -> anyhow::Result<()> {
    let filename = env::args().nth(1).unwrap_or(DEFAULT_FILENAME.to_string());

    let file_contents = fs::read_to_string(filename)?;

    let toml_ext_data: Extensions = toml::from_str(&file_contents)?;

    let datavector = toml_data_to_struct(&toml_ext_data.extensions);

    let responses = get_mozilla_api_responses(datavector)?;

    let mut outfile = fs::File::create("sources.nix")?;

    use std::io::Write;
    outfile.write_all(construct_file(responses).as_bytes())?;

    Ok(())
}

fn construct_file(extensions: Vec<ExtensionData>) -> String {
    let addon_constructed = extensions
        .iter()
        .map(|extension| {
            format!(
                "  (pkgs.fetchFirefoxAddon {{\
                 \n    name = \"{}\";          \
                 \n    url = \"{}\";           \
                 \n    sha256 = \"{}\";        \
                 \n  }})",
                extension.name, extension.url, extension.hash
            )
        })
        .collect::<Vec<String>>().join("\n");

    format!("pkgs: [\n{}\n]", addon_constructed.to_string())
}

// types for turning hashmap into TomlLists
pub struct TomlList {
    human_name: String,
    api_slug: String,
}

pub struct ExtensionData {
    name: String,
    url: String,
    hash: String,
}

fn toml_data_to_struct(tomldata: &HashMap<String, String>) -> Vec<TomlList> {
    let mut structured_data: Vec<TomlList> = Vec::with_capacity(tomldata.len());
    for (key, value) in tomldata {
        structured_data.push(TomlList {
            human_name: key.to_string(),
            api_slug: value.to_string(),
        });
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

fn get_mozilla_api_responses(exts: Vec<TomlList>) -> Result<Vec<ExtensionData>, reqwest::Error> {
    let api = "https://addons.mozilla.org/api/v4/addons/addon";
    let client = reqwest::blocking::Client::new();
    let mut gathered_extensions: Vec<ExtensionData> = Vec::with_capacity(exts.len());

    // check to see if site is functional
    // addons.mozilla.org/api/v4/site (check)
    get_mozilla_api_up();

    for element in exts.iter() {
        let client = &client;
        let request_url: String = format!("{}/{}/", api, element.api_slug);

        let resp = client.get(request_url).send()?.json::<Root>()?;

        gathered_extensions.push(ExtensionData {
            name: element.human_name.to_string(),
            url: resp.current_version.files[0].url.to_string(),
            // slice off "sha256:" from hash string
            hash: resp.current_version.files[0].hash[7..].to_string(),
        });
    }

    return Ok(gathered_extensions);
}
