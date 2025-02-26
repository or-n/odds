use crate::fortuna::{live::URL, COOKIE_ACCEPT};
use crate::utils::{browser, download, page::Tag};

pub struct Page;

impl download::Download<fantoccini::Client, ()> for Tag<Page, String> {
    type Error = fantoccini::error::CmdError;

    async fn download(
        client: &mut fantoccini::Client,
        _data: (),
    ) -> Result<Self, Self::Error> {
        let url = format!("{}/sports/LPLTENNIS", URL);
        browser::download_html(client, url.as_str(), COOKIE_ACCEPT)
            .await
            .map(Tag::new)
    }
}
