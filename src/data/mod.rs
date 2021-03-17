mod service;

pub use service::{get_tennis_matches, get_tennis_players};

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct TennisPlayer {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub hand: bool,
    pub birthday: String,
    pub country_code: String,
}

impl TennisPlayer {
    pub fn new(line: &str) -> TennisPlayer {
        let columns = line.split(',').collect::<Vec<&str>>();

        TennisPlayer {
            id: columns[0].parse().unwrap(),
            first_name: columns[1].parse().unwrap(),
            last_name: columns[2].parse().unwrap(),
            hand: matches!(columns[3], "R"),
            birthday: if columns[4].len() == 8 {
                format!(
                    "{}.{}.{}",
                    &columns[4][6..],
                    &columns[4][4..6],
                    &columns[4][..4],
                )
            } else {
                String::new()
            },
            country_code: columns[5].parse().unwrap(),
        }
    }
}

impl Display for TennisPlayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} ({}-handed, born on {} from {})",
            self.first_name,
            self.last_name,
            if self.hand { "right" } else { "left" },
            self.birthday,
            self.country_code
        )
    }
}

#[derive(Debug)]
pub struct Tourney {
    pub id: String,
    pub name: String,
    pub surface: String,
    pub draw_size: String,
    pub level: String,
    pub date: String,
}

impl Tourney {
    pub fn new(data: Vec<&str>) -> Tourney {
        Tourney {
            id: data[0].parse().unwrap(),
            name: data[1].parse().unwrap(),
            surface: data[2].parse().unwrap(),
            draw_size: data[3].parse().unwrap(),
            level: data[4].parse().unwrap(),
            date: if data[5].len() == 8 {
                format!("{}.{}.{}", &data[5][6..], &data[5][4..6], &data[5][..4],)
            } else {
                String::new()
            },
        }
    }
}

impl Display for Tourney {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} on {} (with a draw size of {} at level {} on date {})",
            self.name, self.surface, self.draw_size, self.level, self.date,
        )
    }
}

#[derive(Debug)]
pub struct Match {
    pub id: i32,
    pub tourney_id: String,
    pub winner_id: i32,
    pub loser_id: i32,
    pub score: String,
    pub best_of: i32,
    pub round: String,
    pub length: u16,
}

impl Match {
    pub fn new(tourney_id: String, winner_id: i32, loser_id: i32, data: Vec<&str>) -> Match {
        Match {
            tourney_id,
            winner_id,
            loser_id,
            id: data[0].parse().unwrap(),
            score: data[1].parse().unwrap(),
            best_of: data[2].parse().unwrap_or_default(),
            round: data[3].parse().unwrap(),
            length: data[4].parse().unwrap_or_default(),
        }
    }
}

impl Display for Match {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "match no {} with a score of {} in a best of {} (round {}, length in minutes {})",
            self.id, self.score, self.best_of, self.round, self.length
        )
    }
}
