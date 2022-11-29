use axum::{extract::Query, response::Html};
use rand::{thread_rng, Rng};
use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct RangeParams {
    start: usize,
    end: usize,
}

pub async fn range_randnum(Query(range): Query<RangeParams>) -> Html<String> {
    tracing::info!("{:?}", range);
    let randnum = thread_rng().gen_range(range.start..range.end);

    Html(format!("<h1>Random Number: {}</h1>", randnum))
}