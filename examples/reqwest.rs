use reqwest::{self, header, Client, Response};
use std::collections::HashMap;

#[allow(dead_code)]
fn blocking_get(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?.json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);

    Ok(())
}

async fn get(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resp1: Response = reqwest::get(url).await?;
    let text = resp1.text().await?;

    let resp2: Response = reqwest::get(url).await?;
    let jsonstr = resp2.json::<HashMap<String, String>>().await?;
    println!("get text: {:#?}", text);
    println!("get json text: {:#?}", jsonstr);

    Ok(())
}

async fn post_body(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = reqwest::Client::new();
    let res: Response = client
        .post(url)
        .body("the exact body that is sent")
        .send()
        .await?;
    println!("post body response:\n--------------{:#?}", res);

    Ok(())
}

async fn post_form(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // This will POST a body of `foo=bar&baz=quux`
    let params = [("foo", "bar"), ("baz", "quux")];
    let client: Client = reqwest::Client::new();
    let res: Response = client.post(url).form(&params).send().await?;
    println!("post form response:\n--------------{:#?}", res);

    Ok(())
}

async fn post_json(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // This will POST a body of `{"lang":"rust","body":"json"}`
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client: Client = reqwest::Client::new();
    let res: Response = client.post(url).json(&map).send().await?;
    println!("post json response:\n--------------{:#?}", res);

    Ok(())
}

async fn post_json_with_headers(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    // 为我们的 http 客户端添加一些缺省的 http 头
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
    let client: Client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // This will POST a body of `{"lang":"rust","body":"json"}`
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let res: Response = client.post(url).json(&map).send().await?;
    println!("post json response with headers:\n--------------{:#?}", res);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 阻塞 get
    // blocking_get("http://httpbin.org/ip")?;

    // 非阻塞 get
    get("http://httpbin.org/ip").await?;

    // 非阻塞 post body
    post_body("http://httpbin.org/post").await?;

    // 非阻塞 post form
    post_form("http://httpbin.org/post").await?;

    // 非阻塞 post json
    post_json("http://httpbin.org/post").await?;

    // 非阻塞 post json with headers
    post_json_with_headers("http://httpbin.org/post").await?;

    Ok(())
}