use colour::{dark_blue, yellow};
use dotenv::dotenv;
use std::error::Error;

use newsapi::{get_articles, Articles};

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        dark_blue!("> {}\n", a.title);
        yellow!("> {}\n", a.url);
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY")?;
    let url: &str = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";
    let url = format!("{}{}", url, api_key);
    let articles = get_articles(&url)?;
    render_articles(&articles);
    Ok(())
}
