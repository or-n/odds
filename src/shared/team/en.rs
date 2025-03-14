pub enum EN {
    ManCity,
    Nottingham,
    Southampton,
    Chelsea,
    Newcastle,
    Liverpool,
    Arsenal,
    ManUnited,
    Everton,
    Wolverhampton,
    AstonVilla,
    Brentford,
    Fulham,
    Brighton,
    Bournemouth,
    Tottenham,
    CrystalPalace,
    Ipswich,
    Leicester,
    WestHam,
    Preston,
    Charlton,
    Morecambe,
    SheffieldUnited,
    Cardiff,
    StockportCounty,
    WestBromwich,
    Plymouth,
    Bristol,
    Leyton,
    Derby,
    QPR,
    Watford,
    Leeds,
    Harrogate,
    Accrington,
    Hull,
    Doncaster,
    Middlesbrough,
    Blackburn,
    Mansfield,
    Wigan,
    Salford,
    Millwall,
    DagenhamAndRed,
    Norwich,
    Bromley,
    Coventry,
    SheffieldWed,
    Reading,
    Burnley,
    Wycombe,
    Portsmouth,
    Tamworth,
    BristolCity,
    LutonTown,
    Exeter,
    OxfordUtd,
    Swansea,
    Birmingham,
    Lincoln,
    Sunderland,
    Stoke,
    Peterborough,
}

pub fn normal() -> Vec<(&'static str, EN)> {
    use EN::*;
    vec![
        ("Nottingham", Nottingham),
        ("Southampton", Southampton),
        ("Chelsea", Chelsea),
        ("Newcastle", Newcastle),
        ("Liverpool", Liverpool),
        ("Arsenal", Arsenal),
        ("Everton", Everton),
        ("Wolverhampton", Wolverhampton),
        ("Brentford", Brentford),
        ("Fulham", Fulham),
        ("Brighton", Brighton),
        ("Bournemouth", Bournemouth),
        ("Tottenham", Tottenham),
        ("Ipswich", Ipswich),
        ("Leicester", Leicester),
        ("Preston", Preston),
        ("Charlton", Charlton),
        ("Morecambe", Morecambe),
        ("Cardiff", Cardiff),
        ("Plymouth", Plymouth),
        ("Bristol", Bristol),
        ("Leyton", Leyton),
        ("Derby", Derby),
        ("Watford", Watford),
        ("Leeds", Leeds),
        ("Harrogate", Harrogate),
        ("Accrington", Accrington),
        ("Hull", Hull),
        ("Doncaster", Doncaster),
        ("Middlesbrough", Middlesbrough),
        ("Blackburn", Blackburn),
        ("Mansfield", Mansfield),
        ("Wigan", Wigan),
        ("Salford", Salford),
        ("Millwall", Millwall),
        ("Norwich", Norwich),
        ("Bromley", Bromley),
        ("Coventry", Coventry),
        ("Reading", Reading),
        ("Burnley", Burnley),
        ("Wycombe", Wycombe),
        ("Portsmouth", Portsmouth),
        ("Tamworth", Tamworth),
        ("Exeter", Exeter),
        ("Swansea", Swansea),
        ("Birmingham", Birmingham),
        ("Lincoln", Lincoln),
        ("Sunderland", Sunderland),
        ("Stoke", Stoke),
        ("Peterborough", Peterborough),
    ]
}

pub fn space() -> Vec<(&'static str, EN)> {
    use EN::*;
    vec![
        ("Aston Villa", AstonVilla),
        ("Crystal Palace", CrystalPalace),
        ("West Ham", WestHam),
        ("Sheffield United", SheffieldUnited),
        ("Stockport County", StockportCounty),
        ("West Bromwich", WestBromwich),
        ("Bristol City", BristolCity),
        ("Luton Town", LutonTown),
    ]
}
