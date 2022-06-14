use anyhow::{anyhow, Result};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
