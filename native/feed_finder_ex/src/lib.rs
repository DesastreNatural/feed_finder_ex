use feedfinder::{Feed, detect_feeds};
use feedfinder::Url;

#[rustler::nif]
fn find_feeds(url: String, html_data: String) -> Vec<String> {
    let p_url = Url::parse(&url).expect("unable to parse url");
    let void: Vec<String> = Vec::new();
    match detect_feeds(&p_url,&html_data) {
        Ok(feeds) => { return feeds.iter().map(|x| x.url().to_string()).collect(); }
        Err(_err) => { return void; }
    }
}

rustler::init!("Elixir.FeedFinderEx", [find_feeds]);
