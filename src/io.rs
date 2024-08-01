use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{anyhow, Result};

use crate::types::{Difficulty, Score};

fn get_leaderboard_path() -> Option<PathBuf> {
    let mut path = home::home_dir()?;
    if cfg!(windows) {
        path.push(Path::new("AppData"));
        path.push(Path::new("Local"));
    } else {
        path.push(Path::new(".config"));
    }
    path.push(Path::new("termsweeper"));
    Some(path)
}

fn parse_line(str: &str) -> Option<Score> {
    if let Some((difficulty, time)) = str.split_once(": ") {
        let difficulty = match difficulty {
            "Easy" => Some(Difficulty::Easy),
            "Medium" => Some(Difficulty::Medium),
            "Hard" => Some(Difficulty::Hard),
            _ => None,
        };
        let time = time.parse().ok();
        if let (Some(difficulty), Some(time)) = (difficulty, time) {
            let time = Duration::from_secs(time);
            Some(Score::new(difficulty, time))
        } else {
            None
        }
    } else {
        None
    }
}

pub fn read_leaderboard() -> Option<Vec<Score>> {
    let mut path = get_leaderboard_path()?;
    path.push(Path::new("leaderboard.txt"));
    let path = Path::new(&path);
    File::open(path).map_or(None, |input| {
        let buffered = BufReader::new(input);
        let mut scores = Vec::new();
        for line in buffered.lines() {
            let score = line.ok().and_then(|line| parse_line(&line));
            if let Some(score) = score {
                scores.push(score);
            }
        }
        if scores.is_empty() {
            None
        } else {
            Some(scores)
        }
    })
}

pub fn write_leaderboard(leaderboard: &[Score]) -> Result<()> {
    // Combine new and old leaderboards, convert to strings
    let mut existing = read_leaderboard().unwrap_or_else(|| Vec::with_capacity(leaderboard.len()));
    leaderboard
        .iter()
        .copied()
        .for_each(|score| existing.push(score));
    let mut existing: Vec<String> = existing
        .into_iter()
        .map(|score| score.as_string())
        .collect();

    // Remove duplicates
    existing.sort_unstable();
    existing.dedup();

    // Ensure the folder and file exist
    let mut path = get_leaderboard_path().ok_or_else(|| anyhow!("No config folder"))?;
    fs::create_dir_all(Path::new(&path))?;
    path.push(Path::new("leaderboard.txt"));
    let file = Path::new(&path);

    let mut file = if file.exists() {
        OpenOptions::new().write(true).open(file)?
    } else {
        File::create(file)?
    };
    for mut score in existing {
        score.push('\n');
        file.write_all(score.as_bytes())?;
    }

    Ok(())
}
