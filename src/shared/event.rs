pub mod football;

use crate::shared::db::ToDBRecord;
use crate::utils::{date, scrape::split2};
use eat::*;
use std::fmt::{Debug, Display};

fn eat_pair(i: &str) -> Result<(&str, (String, String)), ()> {
    let i = '('.drop(i)?;
    let i = '\"'.drop(i)?;
    let Some((a, i)) = i.split_once('\"') else {
        return Err(());
    };
    let i = ','.drop(i)?;
    let i = ' '.drop(i)?;
    let Some((b, i)) = i.split_once(')') else {
        return Err(());
    };
    let pair = (a.to_string(), b.to_string());
    Ok((i, pair))
}

#[derive(Debug, Clone)]
pub struct Event<T1, T2> {
    pub id: T1,
    pub odds: Vec<(T2, f32)>,
}

#[derive(Debug)]
pub struct Match<T1, T2> {
    pub url: String,
    pub date: chrono::NaiveDateTime,
    pub players: [String; 2],
    pub events: Vec<Event<T1, T2>>,
}

impl<T1, T2> Match<T1, T2> {
    pub fn get_id(&self) -> String {
        let p1 = sanitize(self.players[0].as_str());
        let p2 = sanitize(self.players[1].as_str());
        format!("{p1}_vs_{p2}")
    }
}

fn sanitize(x: &str) -> String {
    x.chars()
        .map(|c| match c {
            'ą' => 'a',
            'ć' => 'c',
            'ę' => 'e',
            'ł' => 'l',
            'ń' => 'n',
            'ó' => 'o',
            'ś' => 's',
            'ź' => 'z',
            'ż' => 'z',
            'Ą' => 'A',
            'Ć' => 'C',
            'Ę' => 'E',
            'Ł' => 'L',
            'Ń' => 'N',
            'Ó' => 'O',
            'Ś' => 'S',
            'Ź' => 'Z',
            'Ż' => 'Z',
            ' ' => '_',
            'á' => 'a',
            _ => c,
        })
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect()
}

pub fn eat_match(i: &str) -> Result<Match<String, String>, ()> {
    let parts: Vec<_> = i.split("\n\n").collect();
    let url = parts[0].to_string();
    let date = date::eat2(parts[1]).ok_or(())?;
    let players = split2(parts[2].to_string(), "\n").ok_or(())?;
    let events = parts[3..].into_iter().filter_map(|part| {
        let lines: Vec<_> = part.split('\n').collect();
        let odds: Vec<_> = lines[1..]
            .into_iter()
            .filter_map(|line| eat_pair(line).ok())
            .filter_map(|(_, (name, value))| {
                let value: f32 = value.parse().ok()?;
                Some((name, value))
            })
            .collect();
        let id = lines[0].to_string();
        Some(Event { id, odds })
    });
    Ok(Match {
        url,
        date,
        players,
        events: events.collect(),
    })
}

pub fn event_contents<T1, T2>(event: &Event<T1, T2>) -> String
where
    T1: Display,
    T2: Debug,
{
    let odds: Vec<_> = event
        .odds
        .iter()
        .map(|pair| format!("{:?}", pair))
        .collect();
    format!("{}\n{}", event.id, odds.join("\n"))
}

pub fn match_contents<T1, T2>(m: &Match<T1, T2>) -> Option<String>
where
    T1: Display,
    T2: Debug,
{
    let events = m.events.iter().map(event_contents);
    let events: Vec<_> = events.collect();
    if events.is_empty() {
        return None;
    }
    Some(format!(
        "{}\n\n{}\n\n{}\n{}\n\n{}",
        m.url,
        m.date.format("%Y-%m-%d %H:%M").to_string(),
        m.players[0],
        m.players[1],
        events.join("\n\n")
    ))
}

pub fn match_events_to_db<T1: ToDBRecord, T2>(
    m: &Match<T1, T2>,
) -> Vec<String> {
    let events = m.events.iter().map(|x| x.id.to_db_record());
    let events: Option<Vec<_>> = events.collect();
    events.unwrap_or(vec![])
}
