#![windows_subsystem = "windows"]

use std::env::args;
use url::Url;

fn main() {
    program();
}

fn program() {
    let mut url_string = args().skip(1).next().unwrap();

    if let Some((_, rest_part)) = url_string.split_once("redirect:")
    {
        println!("rest_part: {rest_part}");
        url_string = rest_part.to_owned();
    };
    println!("url_string: {url_string}");

    let parsed_url: Url = match Url::parse(url_string.as_str()) {
        Ok(parsed_url) => parsed_url,
        Err(e) => {
            eprintln!("Failed to parse URL: {}", e);
            return;
        }
    };

    println!("Scheme: {}", parsed_url.scheme());
    println!("Host: {:?}", parsed_url.host_str());
    println!("Path: {}", parsed_url.path());
    println!("Query: {:?}", parsed_url.query());
    println!("Fragment: {:?}", parsed_url.fragment());

    let hostname = parsed_url.host_str().unwrap().to_lowercase();

    //https://eur02.safelinks.protection.outlook.com/?url=https%3A%2F%2Fgoto.netcompany.com%2Fcases%2FGTO27%2FNCINFRA%2FLists%2FTasks%2FDispForm.aspx%3FID%3D227059&data=05%7C02%7Ckkmp%40netcompany.com%7C2fd97a3125ab4882484e08dd4f3dd403%7C8f9b88a73f3e4be3aae42006d4c42306%7C1%7C0%7C638753849148331816%7CUnknown%7CTWFpbGZsb3d8eyJFbXB0eU1hcGkiOnRydWUsIlYiOiIwLjAuMDAwMCIsIlAiOiJXaW4zMiIsIkFOIjoiTWFpbCIsIldUIjoyfQ%3D%3D%7C0%7C%7C%7C&sdata=SQdAjv47e5cfSC%2BD1PpxT2%2FLFO0TcFJYjAhMaGtDGVc%3D&reserved=0
    if hostname.ends_with("safelinks.protection.outlook.com") {
        let query_pairs: Vec<String> = parsed_url
            .query_pairs()
            .map(|(key, value)| format!("'{key}'='{value}'"))
            .collect();

        println!("it is microsoft safelink. {query_pairs:?}");

        if let Some((_, url)) = parsed_url
            .query_pairs()
            .find(|(key, _)| key.to_lowercase() == "url")
        {
            println!("Safe link. OpenUrl: {url:?}");

            let parsed_url: Url = match Url::parse(url.as_ref()) {
                Ok(parsed_url) => parsed_url,
                Err(e) => {
                    eprintln!("Failed to parse URL: {}", e);
                    return;
                }
            };

            let hostname = parsed_url.host_str().unwrap().to_lowercase();
            if hostname.ends_with("goto.netcompany.com") {
                let result = open::with(parsed_url.to_string(), "msedge");

                match result {
                    Ok(_) => (),
                    Err(error) => println!("Error: {error:?}"),
                }

                return;
            }
        };
    }

    if hostname.ends_with("goto.netcompany.com") {
        let result = open::with(url_string, "msedge");

        match result {
            Ok(_) => (),
            Err(error) => println!("Error: {error:?}"),
        }

        return;
    }

    println!("Open {url_string} in firefox");
    let result = open::with(url_string, "firefox");

    match result {
        Ok(_) => (),
        Err(error) => println!("Error: {error:?}"),
    }
}
