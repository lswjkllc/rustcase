use reqwest::StatusCode;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use url::{Position, Url};

/// 获取 base url
fn get_base_url(url: &Url, doc: &Document) -> Result<Url, Box<dyn std::error::Error>> {
    let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);

    let base_url =
        base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;

    Ok(base_url)
}

/// 检查单个链接
fn check_link(url: &Url) -> Result<bool, Box<dyn std::error::Error>> {
    println!("Checking url: {}", url);
    let res = reqwest::blocking::get(url.as_ref())?;

    Ok(res.status() != StatusCode::NOT_FOUND)
}

/// 检查所有链接
fn check_links() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("https://www.rust-lang.org/en-US/")?;

    let res = reqwest::blocking::get(url.as_ref())?;
    let document = Document::from_read(res)?;

    let base_url = get_base_url(&url, &document)?;

    let base_parser = Url::options().base_url(Some(&base_url));

    let links: HashSet<Url> = document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter_map(|link| base_parser.parse(link).ok())
        .collect();
    println!("Link total: {}", links.len());

    links
        .iter()
        .filter(|link| check_link(link).ok() == Some(false))
        .for_each(|x| println!("{} is broken.", x));

    Ok(())
}

fn extract_links() -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get("https://www.rust-lang.org/en-US/")?;

    Document::from_read(res)?
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}

fn main() {
    // 从网页 HTML 中，提取所有链接
    let _ = extract_links();
    // 检查网页是否，有断开的链接
    let _ = check_links();
}
