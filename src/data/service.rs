use std::{collections::HashMap, fs};

use super::{Match, TennisPlayer, Tourney};

pub fn get_tennis_players(path: &str) -> HashMap<i32, TennisPlayer> {
    let mut players = HashMap::<i32, TennisPlayer>::new();
    for line in fs::read_to_string(format!("{}/atp_players.csv", path))
        .expect("Could not read players' file!")
        .lines()
    {
        let user = TennisPlayer::new(line);

        players.insert(user.id, user);
    }

    players
}

fn get_single_file_matches(
    path: &str,
    filename: String,
    tourneys: &mut HashMap<String, Tourney>,
    matches: &mut HashMap<String, Match>,
) {
    for line in fs::read_to_string(format!("{}/{}", path, filename))
        .unwrap_or_default()
        .lines()
        .skip(1)
    {
        let columns = line.split(',').collect::<Vec<&str>>();
        let tourney = tourneys
            .entry(columns[0].parse().unwrap())
            .or_insert_with(|| Tourney::new(Vec::from(&columns[..6])));

        let mut match_data = Vec::from(&columns[23..27]);
        match_data.insert(0, &columns[6]);

        let new_match = Match::new(
            tourney.id.clone(),
            columns[7].parse().unwrap_or_default(),
            columns[15].parse().unwrap(),
            match_data,
        );

        matches.insert(format!("{}:{}", tourney.id, new_match.id), new_match);
    }
}

pub fn get_tennis_matches(path: &str) -> (HashMap<String, Tourney>, HashMap<String, Match>) {
    let mut tourneys = HashMap::<String, Tourney>::new();
    let mut matches = HashMap::<String, Match>::new();

    let files = fs::read_dir(format!("{}/", path))
        .unwrap()
        .map(|path| path.unwrap().file_name().into_string().unwrap_or_default())
        .filter(|filename| filename.contains("atp_matches") && filename.len() == 20);

    for file in files {
        get_single_file_matches(path, file, &mut tourneys, &mut matches);
    }

    (tourneys, matches)
}
