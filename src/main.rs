//use toml::Value;
use std::env;
use std::fs;

static default_filename: &'static str = "./extensions.toml";

fn main() {
    let filename:&str;

    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        filename = &args[1];
    } else {
        filename = default_filename;
    }

    let contents = match fs::read_to_string(filename) {
        Ok(String) => {
            String
        }
        Err(e) => {
            println!("{}", e);
            std::process::exit(-1);
        }
    };

    println!("{}", contents);
}

//fn get_firefox_url() -> Result<String, request::Error> {}
