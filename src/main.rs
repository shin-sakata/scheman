mod pretty_print;
mod schema;
mod to_schema;
mod to_typescript;

use pretty_print::pretty;
use reqwest::blocking as req;
use serde_json::{json, Value};
use to_schema::*;
use to_typescript::to_typescript;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = req::Client::new();
    let body = json!({
        "foo": [10, 20]
    });

    let res: Value = client
        .post("https://httpbin.org/post")
        .json(&body)
        .send()?
        .json()?;

    let schema = res.to_schema();
    let typescript_type = to_typescript(&schema);

    println!("{:#?}", res);
    println!("============================================================");
    println!("{:#?}", schema);
    println!("============================================================");
    println!("{}", &typescript_type);
    println!("============================================================");
    let pretty_type = pretty(&typescript_type)?;
    println!("{}", pretty_type);
    Ok(())
}
