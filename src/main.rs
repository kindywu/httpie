use httpie::*;
use std::collections::HashMap;

use anyhow::Result;
use clap::Parser;
use reqwest::{get, Client};

// cargo run get -u https://www.rust-lang.org
// cargo run post -u http://httpbin.org/post -b a=1 -b b=2
#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    // println!("{:?}", opts);
    let resp = match opts.subcmd {
        SubCommand::Get(Get { url }) => {
            println!("get url {url}");
            get(url).await?
        }
        SubCommand::Post(Post { url, body }) => {
            println!("post url {url} {body:?}");
            let client = Client::new();
            let mut map = HashMap::new();
            for pair in body.iter() {
                map.insert(&pair.k, &pair.v);
            }
            client.post(url).json(&map).send().await?
        }
    };
    print_resp(resp).await
}
