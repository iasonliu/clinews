mod theme;
use std::env;
use std::error::Error;

use newsapi::{Article, Country, Endpoint, NewsAPI};

fn render_articles(articles: &Vec<Article>) {
    let theme = theme::default();
    theme.print_text("# Top Headlines\n\n");
    for a in articles {
        theme.print_text(&format!("`{}`", a.title));
        theme.print_text(&format!("> *{}*", a.url));
        theme.print_text("---")
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("API_KEY")?;
    let mut newsapi = NewsAPI::new(&api_key);
    newsapi
        .endpoint(Endpoint::TopHeadlines)
        .country(Country::Us);
    let newsapi_response = newsapi.fetch()?;
    render_articles(&newsapi_response.articles());
    Ok(())
}
