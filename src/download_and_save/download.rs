use fantoccini::{ClientBuilder, Locator};
use std::time::Duration;
use tokio::time::sleep;

use crate::bookmaker;

pub async fn download<Book>(
    port: u16,
) -> Result<String, fantoccini::error::CmdError>
where
    Book: bookmaker::Site,
{
    let client = ClientBuilder::native()
        .connect(format!("http://localhost:{}", port).as_str())
        .await
        .expect("failed to connect to WebDriver");
    client.goto(Book::SITE).await?;
    let cookie_accept = Locator::Css(Book::COOKIE_ACCEPT_CSS);
    let mut cookie_accepted = false;
    loop {
        let exit = tokio::select! {
            accept = client.wait().for_element(cookie_accept),
            if !cookie_accepted => {
                accept?.click().await?;
                cookie_accepted = true;
                false
            }
            _ = sleep(Duration::from_millis(1000)) => {
                true
            }
        };
        if exit {
            let html = client.source().await?;
            client.close().await?;
            return Ok(html);
        }
    }
}