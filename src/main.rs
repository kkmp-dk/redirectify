#![windows_subsystem = "windows"]

use std::env::args;
use url::Url;

fn main() {
    let url_str = args().skip(1).next().unwrap();

    let hostname: String = match Url::parse(url_str.as_str()) {
        Ok(parsed_url) => {
            println!("Scheme: {}", parsed_url.scheme());
            println!("Host: {:?}", parsed_url.host_str());
            println!("Path: {}", parsed_url.path());
            println!("Query: {:?}", parsed_url.query());
            println!("Fragment: {:?}", parsed_url.fragment());
            parsed_url.host_str().unwrap().to_lowercase()
        }
        Err(e) => {
            eprintln!("Failed to parse URL: {}", e);
            return;
        }
    };

    if hostname.starts_with("goto.netcompany.com") {
        let result = open::with(url_str, "msedge");

        match result {
            Ok(_) => (),
            Err(error) => println!("Error: {error:?}"),
        }

        return;
    }

    println!("Open {url_str} in firefox");
    let result = open::with(url_str, "firefox");

    match result {
        Ok(_) => (),
        Err(error) => println!("Error: {error:?}"),
    }
}
