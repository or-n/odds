use crate::shared::{self, book};
use crate::utils::{self, browser::Browser};

pub struct Book;

impl book::Name for Book {
    const NAME: &'static str = "superbet";
}

pub struct LivePage(String);

impl utils::download::Download for Browser<Book> {
    type Output = Result<LivePage, utils::browser::Error>;
    type Error = fantoccini::error::CmdError;

    async fn download(&self) -> Result<Self::Output, Self::Error> {
        Ok(match utils::browser::client(self.port).await {
            Ok(client) => Ok(LivePage(
                utils::download::download(
                    client,
                    "https://superbet.pl/zaklady-bukmacherskie/live",
                    fantoccini::Locator::Css(
                        r#"button[id="onetrust-accept-btn-handler"]"#,
                    ),
                )
                .await?,
            )),
            Err(connect_error) => Err(connect_error),
        })
    }
}

impl ToString for LivePage {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

use book::Error;

impl book::SportBets for LivePage {
    fn sport_bets(&self) -> Result<Vec<(book::Teams, book::Odds)>, Error> {
        use scraper::Selector;
        let team1 = Selector::parse("div.e2e-event-team1-name").unwrap();
        let team2 = Selector::parse("div.e2e-event-team2-name").unwrap();
        shared::sport_bets::extract(
            &self.0,
            Selector::parse("div.event-card").unwrap(),
            Selector::parse("span.odd-button__odd-value-new").unwrap(),
            |x| {
                Ok([
                    x.select(&team1).next().ok_or(Error::MissingTeam1)?,
                    x.select(&team2).next().ok_or(Error::MissingTeam2)?,
                ])
            },
        )
    }
}
