use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub clocks: Vec<ClockEntry>,
}

#[derive(Deserialize, Clone)]
pub struct ClockEntry {
    pub name: String,
    pub city: String,
    pub timezone: String,
    pub flag: Option<String>,
    pub status: Option<StatusSchedule>,
}

#[derive(Deserialize, Clone)]
pub struct StatusSchedule {
    pub blocks: HashMap<String, StatusBlock>,
    pub months: HashMap<String, String>,
}

#[derive(Deserialize, Clone)]
pub struct StatusBlock {
    pub label: String,
    pub start: String,
    pub end: String,
}

pub enum Availability {
    Busy(String),
    Available,
    DayOff,
}

pub fn config_path() -> PathBuf {
    let home = dirs::home_dir().expect("could not find home directory");
    home.join(".config/fuso/clocks.json")
}

pub fn load_config() -> Config {
    let path = config_path();

    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).ok();
        }
        let json = serde_json::to_string_pretty(&serde_json::json!({
            "clocks": [{
                "name": "Me",
                "city": "New York",
                "timezone": "America/New_York"
            }]
        }))
        .unwrap();
        fs::write(&path, json).ok();
        return Config {
            clocks: vec![ClockEntry {
                name: "Me".into(),
                city: "New York".into(),
                timezone: "America/New_York".into(),
                flag: None,
                status: None,
            }],
        };
    }

    let data = fs::read_to_string(&path).expect("could not read config file");
    serde_json::from_str(&data).expect("invalid config format")
}

fn parse_time(t: &str) -> u32 {
    let parts: Vec<u32> = t.split(':').filter_map(|p| p.parse().ok()).collect();
    if parts.len() == 2 {
        parts[0] * 60 + parts[1]
    } else {
        0
    }
}

pub fn current_availability(
    entry: &ClockEntry,
    now: chrono::DateTime<chrono_tz::Tz>,
) -> Option<Availability> {
    use chrono::{Datelike, Timelike};

    let schedule = entry.status.as_ref()?;
    let month_key = format!("{}-{:02}", now.year(), now.month());
    let month_str = schedule.months.get(&month_key)?;
    let day = now.day() as usize;

    if day < 1 || day > month_str.len() {
        return None;
    }

    let block_id = &month_str[day - 1..day];
    let now_minutes = now.hour() * 60 + now.minute();

    if block_id != "0" {
        if let Some(block) = schedule.blocks.get(block_id) {
            let start = parse_time(&block.start);
            let end = parse_time(&block.end);

            if end > start {
                if now_minutes >= start && now_minutes < end {
                    return Some(Availability::Busy(block.label.clone()));
                }
            } else if end < start && now_minutes >= start {
                return Some(Availability::Busy(block.label.clone()));
            }
        }
    }

    let yesterday = now - chrono::Duration::days(1);
    let y_month_key = format!("{}-{:02}", yesterday.year(), yesterday.month());
    if let Some(y_month_str) = schedule.months.get(&y_month_key) {
        let y_day = yesterday.day() as usize;
        if y_day >= 1 && y_day <= y_month_str.len() {
            let y_block_id = &y_month_str[y_day - 1..y_day];
            if y_block_id != "0" {
                if let Some(y_block) = schedule.blocks.get(y_block_id) {
                    let y_start = parse_time(&y_block.start);
                    let y_end = parse_time(&y_block.end);
                    if y_end < y_start && now_minutes < y_end {
                        return Some(Availability::Busy(y_block.label.clone()));
                    }
                }
            }
        }
    }

    if block_id == "0" {
        Some(Availability::DayOff)
    } else {
        Some(Availability::Available)
    }
}
