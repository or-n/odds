use eat::*;
use fortuna::prematch::football;
use odds::fortuna;
use odds::shared;
use odds::utils::{
    browser, date,
    download::Download,
    page::{Name, Tag, Url},
    save::save,
};
use scraper::Html;
use shared::{book::Subpages, event};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

fn contents(document: Tag<football::subpage::Page, Html>) -> Option<String> {
    let Some(players) = document.players() else {
        return None;
    };
    println!("{} - {}", players[0], players[1]);
    let events = document.events().into_iter().filter_map(|event| {
        let (rest, event_type) =
            event::Football::eat(event.name.as_str(), players.clone()).unwrap();
        use event::Football::*;
        if rest != ""
            || !matches!(
                event_type,
                CornersP1
                    | CornersP1H1
                    | CornersP1H2
                    | CornersP2
                    | CornersP2H1
                    | CornersP2H2
            )
        {
            return None;
        }
        let safe_odds: Vec<_> = event
            .odds
            .into_iter()
            .filter(|(_, x)| *x >= 3.1 && *x <= 3.3)
            .map(|pair| format!("{:?}", pair))
            .collect();
        if safe_odds.is_empty() {
            return None;
        }
        Some(format!("{}\n{}", event.name, safe_odds.join("\n")))
    });
    let events: Vec<_> = events.collect();
    if events.is_empty() {
        return None;
    }
    Some(events.join("\n"))
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
    let total_count = subpages.len();
    let queue = Arc::new(Mutex::new(subpages));
    println!("Elapsed time: {:.2?}", start.elapsed());
    let start = Instant::now();
    let mut download_count = 0;
    let mut save_count = 0;
    while let Some((subpage, date)) = queue.lock().await.pop() {
        if !date::in_days(date, 1) {
            continue;
        }
        let html = Tag::download(&mut client, subpage.clone()).await.unwrap();
        download_count += 1;
        let Some(contents) = contents(html.document()) else {
            continue;
        };
        let file = format!("downloads/{}", subpage.name());
        let f = format!("{}\n\n{}", subpage.url(), contents);
        let _ = save(f.as_bytes(), file).await;
        save_count += 1;
    }
    client.close().await.unwrap();
    let elapsed = start.elapsed().as_secs_f32();
    println!("Elapsed time: {:.2?}", elapsed);
    println!("Total count: {}", total_count);
    println!("Download count: {}", download_count);
    println!("Save count: {}", save_count);
    println!("{:.2?} / download", elapsed / download_count as f32);
}
