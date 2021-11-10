use colour::{dark_blue, yellow};
use std::error::Error;

use newsapi::{get_articles, Articles};

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
