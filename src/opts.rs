use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use clap::Parser;
use reqwest::Url;

#[derive(Debug, Parser)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(about = "Feed get with an url and we will retrieve the response for you")]
    Get(Get),
    #[command(
        about = "Feed post with an url and optional key=value pairs. We will post the data as JSON, and retrieve the response for you"
    )]
    Post(Post),
}

#[derive(Debug, Parser)]
pub struct Get {
    #[arg(short, long, value_parser = verify_url)]
    pub url: String,
}

#[derive(Debug, Parser)]
pub struct Post {
    #[arg(short, long, value_parser = verify_url)]
    pub url: String,
    #[arg(short, long, value_parser = verify_kv_pair)]
    pub body: Vec<KvPair>,
}

fn verify_url(s: &str) -> Result<String> {
    let _ = Url::parse(s)?;
    Ok(s.into())
}

#[derive(Debug, Clone)]
pub struct KvPair {
    pub k: String,
    pub v: String,
}

impl FromStr for KvPair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let mut split = s.split("=");
        // let err = || anyhow!(format!("Failed to parse {}", s));
        // Ok(Self {
        //     k: split.next().ok_or_else(err)?.to_string(),
        //     v: split.next().ok_or_else(err)?.to_string(),
        // })

        let parts: Vec<&str> = s.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid key=value pair: {}", s));
        }
        Ok(Self {
            k: parts[0].to_string(),
            v: parts[1].to_string(),
        })
    }
}

fn verify_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}
