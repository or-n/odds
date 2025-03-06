use crate::shared::event;
use eat::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Tab {
    Winner,
    AsianHandicap,
    EuropeanHandicap,
    Corners,
    TotalGoals,
    DoubleChance,
    Cards,
    IndividualTotalGoals,
    IndividualCorners,
    BothTeamsToScore,
    DrawNoBet,
    ExactGoalsNumber,
    Penalty,
}

pub fn tab(event: &event::Football) -> Option<Tab> {
    use event::Football::*;
    use Tab::*;
    match event {
        Goals | GoalsH1 | GoalsH2 => Some(TotalGoals),
        GoalsP1 | GoalsP1H1 | GoalsP1H2 | GoalsP2 | GoalsP2H1 | GoalsP2H2 => {
            Some(IndividualTotalGoals)
        }
        ExactGoals | ExactGoalsH1 | ExactGoalsH2 => Some(ExactGoalsNumber),
        BothToScore | BothToScoreH1 | BothToScoreH2 => Some(BothTeamsToScore),
        Handicap | HandicapH1 | HandicapH2 => Some(AsianHandicap),
        H1 | H2 => Some(Winner),
        event::Football::Corners | CornersH1 | CornersH2 => Some(Tab::Corners),
        CornersP1 | CornersP1H1 | CornersP1H2 | CornersP2 | CornersP2H1
        | CornersP2H2 => Some(IndividualCorners),
        Unknown(_) => None,
    }
}

pub fn toolbar(event: &event::Football) -> Option<Toolbar> {
    use event::Football::*;
    use Toolbar::*;
    match event {
        Goals => todo!(),
        GoalsH1 => todo!(),
        GoalsH2 => todo!(),
        GoalsP1 => Some(Home(Part::FullTime)),
        GoalsP1H1 => Some(Home(Part::FirstHalf)),
        GoalsP1H2 => Some(Home(Part::SecondHalf)),
        GoalsP2 => Some(Away(Part::FullTime)),
        GoalsP2H1 => Some(Away(Part::FirstHalf)),
        GoalsP2H2 => Some(Away(Part::SecondHalf)),
        ExactGoals => todo!(),
        ExactGoalsH1 => todo!(),
        ExactGoalsH2 => todo!(),
        BothToScore => todo!(),
        BothToScoreH1 => todo!(),
        BothToScoreH2 => todo!(),
        Handicap => todo!(),
        HandicapH1 => todo!(),
        HandicapH2 => todo!(),
        H1 => todo!(),
        H2 => todo!(),
        Corners => todo!(),
        CornersH1 => todo!(),
        CornersH2 => todo!(),
        CornersP1 => todo!(),
        CornersP1H1 => todo!(),
        CornersP1H2 => todo!(),
        CornersP2 => todo!(),
        CornersP2H1 => todo!(),
        CornersP2H2 => todo!(),
        Unknown(_) => todo!(),
    }
}

impl Eat<&str, (), ()> for Tab {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        use Tab::*;
        if let Ok(i) = "1x2".drop(i) {
            return Ok((i, Winner));
        }
        if let Ok(i) = "Asian Handicap".drop(i) {
            return Ok((i, AsianHandicap));
        }
        if let Ok(i) = "European Handicap".drop(i) {
            return Ok((i, EuropeanHandicap));
        }
        if let Ok(i) = "Corners".drop(i) {
            return Ok((i, Corners));
        }
        if let Ok(i) = "Total Goals".drop(i) {
            return Ok((i, TotalGoals));
        }
        if let Ok(i) = "Double Chance".drop(i) {
            return Ok((i, DoubleChance));
        }
        if let Ok(i) = "Cards".drop(i) {
            return Ok((i, Cards));
        }
        if let Ok(i) = "Individual Total Goals".drop(i) {
            return Ok((i, IndividualTotalGoals));
        }
        if let Ok(i) = "Individual Corners".drop(i) {
            return Ok((i, IndividualCorners));
        }
        if let Ok(i) = "Both Teams To Score".drop(i) {
            return Ok((i, BothTeamsToScore));
        }
        if let Ok(i) = "Draw No Bet".drop(i) {
            return Ok((i, DrawNoBet));
        }
        if let Ok(i) = "Exact Goals Number".drop(i) {
            return Ok((i, ExactGoalsNumber));
        }
        if let Ok(i) = "Penalty".drop(i) {
            return Ok((i, Penalty));
        }
        Err(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Toolbar {
    FullTime,
    FirstHalf,
    SecondHalf,
    Winner(Part),
    AsianHandicap(Part),
    Total(Part),
    DoubleChance,
    HomeTotal(Part),
    AwayTotal(Part),
    Home(Part),
    Away(Part),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Part {
    FullTime,
    FirstHalf,
    SecondHalf,
}

impl Eat<&str, (), ()> for Toolbar {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        use Toolbar::*;
        if let Ok(i) = "Full Time".drop(i) {
            return Ok((i, FullTime));
        }
        if let Ok(i) = "1st Half".drop(i) {
            return Ok((i, FirstHalf));
        }
        if let Ok(i) = "2nd Half".drop(i) {
            return Ok((i, SecondHalf));
        }
        if let Ok(i) = "1x2".drop(i) {
            let (i, part) = Part::eat(i, ())?;
            return Ok((i, Winner(part)));
        }
        if let Ok(i) = "Asian Handicap".drop(i) {
            let (i, part) = Part::eat(i, ())?;
            return Ok((i, AsianHandicap(part)));
        }
        if let Ok(i) = "Total".drop(i) {
            let (i, part) = Part::eat(i, ())?;
            return Ok((i, Total(part)));
        }
        if let Ok(i) = "Double Chance".drop(i) {
            return Ok((i, DoubleChance));
        }
        if let Ok(i) = "Home Total".drop(i) {
            let (i, part) = Part::eat(i, ())?;
            return Ok((i, HomeTotal(part)));
        }
        if let Ok(i) = "Away Total".drop(i) {
            let (i, part) = Part::eat(i, ())?;
            return Ok((i, AwayTotal(part)));
        }
        if let Ok(i) = "Home".drop(i) {
            use Part::*;
            if let Ok(i) = " FT".drop(i) {
                return Ok((i, Home(FullTime)));
            }
            if let Ok(i) = " H1".drop(i) {
                return Ok((i, Home(FirstHalf)));
            }
            if let Ok(i) = " H2".drop(i) {
                return Ok((i, Home(SecondHalf)));
            }
        }
        if let Ok(i) = "Away".drop(i) {
            use Part::*;
            if let Ok(i) = " FT".drop(i) {
                return Ok((i, Away(FullTime)));
            }
            if let Ok(i) = " H1".drop(i) {
                return Ok((i, Away(FirstHalf)));
            }
            if let Ok(i) = " H2".drop(i) {
                return Ok((i, Away(SecondHalf)));
            }
        }
        Err(())
    }
}

impl Eat<&str, (), ()> for Part {
    fn eat(i: &str, _data: ()) -> Result<(&str, Self), ()> {
        use Part::*;
        if let Ok(i) = " (H1)".drop(i) {
            return Ok((i, FirstHalf));
        }
        if let Ok(i) = " (H2)".drop(i) {
            return Ok((i, SecondHalf));
        }
        Ok((i, FullTime))
    }
}
