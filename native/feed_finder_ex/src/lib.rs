use std::collections::HashMap;

use feedfinder::{Feed, detect_feeds};
use feedfinder::Url;

fn encode_feed(feed: &Feed) -> HashMap<String, String> {
    let mut encoded: HashMap<String, String> = HashMap::new();
    encoded.insert("url".to_string(),feed.url().to_string());
    encoded.insert("feed_type".to_string(),format!("{:?}",feed.feed_type()));
    encoded.insert("title".to_string(),feed.title().unwrap_or("").to_string());
    return encoded;
}

#[rustler::nif]
fn find_feeds(url: String, html_data: String) -> Result<Vec<HashMap<String, String>>,String> {
    let p_url = Url::parse(&url);
    match p_url {
        Ok(url) => {
            match detect_feeds(&url,&html_data) {
                Ok(feeds) => { return Ok(feeds.iter().map(|x| encode_feed(x)).collect()); }
                Err(_err) => { return Err(format!("{:?}",_err)); }
            }
        }
        Err(_err) => { return Err(format!("{:?}",_err)); }
    }
}

rustler::init!("Elixir.FeedFinderEx", [find_feeds]);
