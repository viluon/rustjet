use anyhow::Result;
use clap::Parser;
use rustjet::{
    apis::{
        configuration::{ApiKey, Configuration},
        users_api::{authenticate, login_registered_account},
    },
    models::{RegisteredLogin, Token},
};
use std::collections::HashMap;

async fn login(code: &str, password: &str) -> Result<Configuration> {
    let credentials = RegisteredLogin {
        account_code: code.to_string(),
        password: password.to_string(),
    };

    let Token { token } =
        login_registered_account(&Default::default(), credentials, None, None, None).await?;

    Ok(Configuration {
        api_key: Some(ApiKey {
            prefix: Some("Bearer".to_string()),
            key: token.unwrap(),
        }),
        ..Default::default()
    })
}

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    code: String,
    #[arg(short, long)]
    password: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let config = login(&args.code, &args.password).await;
    println!("{:#?}", config);

    let user = authenticate(&config?, None, None).await?;
    println!("{:#?}", user);

    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
