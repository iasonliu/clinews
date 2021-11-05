use colour::{dark_blue, yellow};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}
fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response: String = ureq::get(url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;
    Ok(articles)
}

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        dark_blue!("> {}\n", a.title);
        yellow!("> {}\n", a.url);
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let url: &str =
        "https://newsapi.org/v2/top-headlines?country=us&apiKey=3d17acbf6bc749ba908d040a2699089a";
    let articles = get_articles(url)?;
    render_articles(&articles);
    Ok(())
}
