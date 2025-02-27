mod shared;
mod utils;

mod fortuna;
// mod sts;
// mod superbet;

use eat::*;
use fortuna::prematch::football::EventType;
use shared::book::Subpages;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use utils::{
    browser,
    download::Download,
    page::{Name, Tag, Url},
    save::save,
};

fn in_range(x: f32, range: [f32; 2]) -> bool {
    x >= range[0] && x <= range[1]
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let mut client = browser::connect(4444).await;
    let page = fortuna::prematch::football::Page;
    let html = shared::download_and_save::run(&mut client, page)
        .await
        .unwrap();
    let subpages = html.document().subpages();
    let queue = Arc::new(Mutex::new(subpages));
    while let Some(subpage) = queue.lock().await.pop() {
        println!("{}", subpage.url());
        let html = Tag::download(&mut client, subpage.clone()).await.unwrap();
        let events = html.document().events();
        let mut contents = String::new();
        for event in events {
            let safe_odds: Vec<_> = event
                .odds
                .iter()
                .filter(|pair| in_range(pair.1, [3.1, 3.3]))
                .collect();
            if let Ok(("", event_type)) =
                EventType::eat(event.name.as_str(), ())
            {
                if !safe_odds.is_empty()
                    && !matches!(event_type, EventType::Unknown(_))
                {
                    contents.push_str(&format!("{}\n", event.name));
                    for pair in safe_odds {
                        contents.push_str(&format!("{:?}\n", pair));
                    }
                    contents.push_str("\n");
                }
            }
        }
        if !contents.is_empty() {
            let file = format!("downloads/{}", subpage.name());
            let f = format!("{}\n\n{}", subpage.url(), contents);
            let _ = save(f.as_bytes(), file);
        }
    }
    client.close().await.unwrap();
    println!("Elapsed time: {:.2?}", start.elapsed());
}
