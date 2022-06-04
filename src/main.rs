mod term;
use std::io::Write;
use std::str::FromStr;

use std::process::exit;
use term::*;

const NUMERIC: usize = 63700991;

fn main() {
    print!("IP Address: ");
    flush();
    let ip = read_line().trim_end().to_string();

    if let Err(why) = &std::net::IpAddr::from_str(ip.as_str()) {
        eprintln!("Error while parsing IP address: {}", why);
        exit(1);
    }

    let req = reqwest::blocking::get(format!("http://ip-api.com/json/{}?fields={}", ip, NUMERIC));

    if let Err(why) = &req {
        eprintln!("Error while sending request: {}", why);
        exit(1);
    }

    let req = req.unwrap();

    let json = req.json::<serde_json::Value>();

    if let Err(why) = &json {
        eprintln!("Error while parsing JSON: {}", why);
        exit(1);
    }

    let json = json.unwrap();

    let status = json["status"].as_str().unwrap();

    if status == "fail" {
        eprintln!("Error: {}", json);
        exit(1);
    }
    if status == "success" {
        println!("{:#?}", json);
        let mut file = std::fs::File::create(format!("{}.txt", ip)).unwrap();

        file.write(format!("{:#?}", json).as_bytes()).unwrap();

        print!("Saved contents, press ENTER to exit. ");
        flush();
        read_line();
        exit(0);
    }
}
