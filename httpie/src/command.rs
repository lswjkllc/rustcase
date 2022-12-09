use std::str::FromStr;

use clap::{Args, Parser, Subcommand};
use anyhow::{anyhow, Result};
use reqwest::Url;

#[derive(Parser, Debug)]
#[command(name = "Httpie Client", author = "Kust")]
#[command(version = "0.0.1", about = "A little tool")]
pub struct HttpieCli {
    #[command(subcommand)]
    pub subcmd: Action,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Action {
    Get(Get),
    Post(Post),
}

#[derive(Args, Debug, Clone)]
pub struct Get {
    #[arg(value_parser = parse_url)]
    pub url: String,
}

#[derive(Args, Debug, Clone)]
pub struct Post {
    #[arg(value_parser = parse_url)]
    pub url: String,
    #[arg(value_parser = parse_kvpair)]
    pub body: Vec<Kvpair>,
}

/// 相关数据结构
#[derive(Clone, Debug)]
pub struct Kvpair {
    pub k: String,
    pub v: String,
}

impl FromStr for Kvpair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

/// 相关解析函数
fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

fn parse_kvpair(s: &str) -> Result<Kvpair> {
    Ok(s.parse()?)
}