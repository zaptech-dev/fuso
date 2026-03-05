use chrono::{Datelike, Timelike};
use chrono_tz::Tz;
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

pub fn timezone_to_flag(tz: &str) -> &'static str {
    match tz {
        s if s.starts_with("America/New_York")
            | s.starts_with("America/Chicago")
            | s.starts_with("America/Denver")
            | s.starts_with("America/Los_Angeles")
            | s.starts_with("America/Phoenix")
            | s.starts_with("America/Anchorage")
            | s.starts_with("Pacific/Honolulu") =>
        {
            "\u{1f1fa}\u{1f1f8}"
        }
        s if s.starts_with("America/Sao_Paulo")
            | s.starts_with("America/Fortaleza")
            | s.starts_with("America/Manaus")
            | s.starts_with("America/Bahia")
            | s.starts_with("America/Belem")
            | s.starts_with("America/Recife")
            | s.starts_with("America/Cuiaba")
            | s.starts_with("America/Campo_Grande")
            | s.starts_with("America/Rio_Branco")
            | s.starts_with("America/Porto_Velho")
            | s.starts_with("America/Maceio")
            | s.starts_with("America/Araguaina") =>
        {
            "\u{1f1e7}\u{1f1f7}"
        }
        "Asia/Tokyo" => "\u{1f1ef}\u{1f1f5}",
        "Europe/London" | "Europe/Dublin" => "\u{1f1ec}\u{1f1e7}",
        "Europe/Paris" => "\u{1f1eb}\u{1f1f7}",
        "Europe/Berlin" => "\u{1f1e9}\u{1f1ea}",
        "Europe/Rome" => "\u{1f1ee}\u{1f1f9}",
        "Europe/Madrid" => "\u{1f1ea}\u{1f1f8}",
        "Europe/Lisbon" => "\u{1f1f5}\u{1f1f9}",
        "Europe/Amsterdam" => "\u{1f1f3}\u{1f1f1}",
        "Europe/Zurich" => "\u{1f1e8}\u{1f1ed}",
        "Europe/Vienna" => "\u{1f1e6}\u{1f1f9}",
        "Europe/Prague" => "\u{1f1e8}\u{1f1ff}",
        "Europe/Warsaw" => "\u{1f1f5}\u{1f1f1}",
        "Europe/Stockholm" => "\u{1f1f8}\u{1f1ea}",
        "Europe/Oslo" => "\u{1f1f3}\u{1f1f4}",
        "Europe/Copenhagen" => "\u{1f1e9}\u{1f1f0}",
        "Europe/Helsinki" => "\u{1f1eb}\u{1f1ee}",
        "Europe/Moscow" => "\u{1f1f7}\u{1f1fa}",
        "Europe/Istanbul" => "\u{1f1f9}\u{1f1f7}",
        "Asia/Shanghai" => "\u{1f1e8}\u{1f1f3}",
        "Asia/Hong_Kong" => "\u{1f1ed}\u{1f1f0}",
        "Asia/Seoul" => "\u{1f1f0}\u{1f1f7}",
        "Asia/Singapore" => "\u{1f1f8}\u{1f1ec}",
        "Asia/Kolkata" => "\u{1f1ee}\u{1f1f3}",
        "Asia/Dubai" => "\u{1f1e6}\u{1f1ea}",
        "Asia/Bangkok" => "\u{1f1f9}\u{1f1ed}",
        "Asia/Jakarta" => "\u{1f1ee}\u{1f1e9}",
        "Asia/Taipei" => "\u{1f1f9}\u{1f1fc}",
        "Asia/Riyadh" => "\u{1f1f8}\u{1f1e6}",
        "Asia/Jerusalem" => "\u{1f1ee}\u{1f1f1}",
        "Australia/Sydney" | "Australia/Melbourne" | "Australia/Perth" | "Australia/Brisbane" => {
            "\u{1f1e6}\u{1f1fa}"
        }
        "Pacific/Auckland" => "\u{1f1f3}\u{1f1ff}",
        "America/Toronto" | "America/Vancouver" | "America/Edmonton" => "\u{1f1e8}\u{1f1e6}",
        "America/Mexico_City" => "\u{1f1f2}\u{1f1fd}",
        "America/Argentina/Buenos_Aires" => "\u{1f1e6}\u{1f1f7}",
        "America/Santiago" => "\u{1f1e8}\u{1f1f1}",
        "America/Bogota" => "\u{1f1e8}\u{1f1f4}",
        "America/Lima" => "\u{1f1f5}\u{1f1ea}",
        "Africa/Johannesburg" => "\u{1f1ff}\u{1f1e6}",
        "Africa/Lagos" => "\u{1f1f3}\u{1f1ec}",
        "Africa/Cairo" => "\u{1f1ea}\u{1f1ec}",
        "Africa/Nairobi" => "\u{1f1f0}\u{1f1ea}",
        _ => "\u{1f30d}",
    }
}

fn parse_time(t: &str) -> u32 {
    let parts: Vec<u32> = t.split(':').filter_map(|p| p.parse().ok()).collect();
    if parts.len() == 2 {
        parts[0] * 60 + parts[1]
    } else {
        0
    }
}

pub fn current_availability(entry: &ClockEntry, now: chrono::DateTime<Tz>) -> Option<Availability> {
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

pub fn relative_offset(local_tz: Tz, remote_tz: Tz, now: chrono::DateTime<chrono::Utc>) -> String {
    use chrono::Offset;
    let local_offset = now.with_timezone(&local_tz).offset().fix().local_minus_utc();
    let remote_offset = now.with_timezone(&remote_tz).offset().fix().local_minus_utc();
    let diff = remote_offset - local_offset;
    let hours = diff / 3600;
    let minutes = (diff.abs() % 3600) / 60;

    if hours == 0 && minutes == 0 {
        return "local".into();
    }
    if minutes > 0 {
        format!("{:+}:{:02}h", hours, minutes)
    } else {
        format!("{:+}h", hours)
    }
}

pub fn local_tz() -> Tz {
    iana_time_zone::get_timezone()
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(chrono_tz::UTC)
}
