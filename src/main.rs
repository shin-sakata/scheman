mod schema;

use reqwest::blocking as req;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp: HashMap<String, String> = req::get("https://httpbin.org/ip")?.json()?;
    println!("{:#?}", resp);
    Ok(())
}
