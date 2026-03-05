use chrono::{Datelike, Timelike};
use chrono_tz::Tz;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Config {
    clocks: Vec<ClockEntry>,
}

#[derive(Deserialize)]
struct ClockEntry {
    name: String,
    city: String,
    timezone: String,
    flag: Option<String>,
    status: Option<StatusSchedule>,
}

#[derive(Deserialize)]
struct StatusSchedule {
    blocks: HashMap<String, StatusBlock>,
    months: HashMap<String, String>,
}

#[derive(Deserialize)]
struct StatusBlock {
    label: String,
    start: String,
    end: String,
}

enum Availability {
    Busy(String),
    Available,
    DayOff,
}

fn config_path() -> PathBuf {
    let home = dirs::home_dir().expect("could not find home directory");
    home.join(".config/fuso/clocks.json")
}

fn default_config() -> Config {
    Config {
        clocks: vec![ClockEntry {
            name: "Me".into(),
            city: "New York".into(),
            timezone: "America/New_York".into(),
            flag: None,
            status: None,
        }],
    }
}

fn load_config() -> Config {
    let path = config_path();

    if !path.exists() {
        let config = default_config();
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
        return config;
    }

    let data = fs::read_to_string(&path).expect("could not read config file");
    serde_json::from_str(&data).expect("invalid config format")
}

fn timezone_to_flag(tz: &str) -> &'static str {
    match tz {
        s if s.starts_with("America/New_York")
            | s.starts_with("America/Chicago")
            | s.starts_with("America/Denver")
            | s.starts_with("America/Los_Angeles")
            | s.starts_with("America/Phoenix")
            | s.starts_with("America/Anchorage")
            | s.starts_with("Pacific/Honolulu") =>
        {
            "🇺🇸"
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
            "🇧🇷"
        }
        "Asia/Tokyo" => "🇯🇵",
        "Europe/London" | "Europe/Dublin" => "🇬🇧",
        "Europe/Paris" => "🇫🇷",
        "Europe/Berlin" => "🇩🇪",
        "Europe/Rome" => "🇮🇹",
        "Europe/Madrid" => "🇪🇸",
        "Europe/Lisbon" => "🇵🇹",
        "Europe/Amsterdam" => "🇳🇱",
        "Europe/Zurich" => "🇨🇭",
        "Europe/Vienna" => "🇦🇹",
        "Europe/Prague" => "🇨🇿",
        "Europe/Warsaw" => "🇵🇱",
        "Europe/Stockholm" => "🇸🇪",
        "Europe/Oslo" => "🇳🇴",
        "Europe/Copenhagen" => "🇩🇰",
        "Europe/Helsinki" => "🇫🇮",
        "Europe/Moscow" => "🇷🇺",
        "Europe/Istanbul" => "🇹🇷",
        "Asia/Shanghai" => "🇨🇳",
        "Asia/Hong_Kong" => "🇭🇰",
        "Asia/Seoul" => "🇰🇷",
        "Asia/Singapore" => "🇸🇬",
        "Asia/Kolkata" => "🇮🇳",
        "Asia/Dubai" => "🇦🇪",
        "Asia/Bangkok" => "🇹🇭",
        "Asia/Jakarta" => "🇮🇩",
        "Asia/Taipei" => "🇹🇼",
        "Asia/Riyadh" => "🇸🇦",
        "Asia/Jerusalem" => "🇮🇱",
        "Australia/Sydney" | "Australia/Melbourne" | "Australia/Perth" | "Australia/Brisbane" => {
            "🇦🇺"
        }
        "Pacific/Auckland" => "🇳🇿",
        "America/Toronto" | "America/Vancouver" | "America/Edmonton" => "🇨🇦",
        "America/Mexico_City" => "🇲🇽",
        "America/Argentina/Buenos_Aires" => "🇦🇷",
        "America/Santiago" => "🇨🇱",
        "America/Bogota" => "🇨🇴",
        "America/Lima" => "🇵🇪",
        "Africa/Johannesburg" => "🇿🇦",
        "Africa/Lagos" => "🇳🇬",
        "Africa/Cairo" => "🇪🇬",
        "Africa/Nairobi" => "🇰🇪",
        _ => "🌍",
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

fn current_availability(entry: &ClockEntry, now: chrono::DateTime<Tz>) -> Option<Availability> {
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

    // Check yesterday's cross-midnight block
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

fn relative_offset(local_tz: Tz, remote_tz: Tz, now: chrono::DateTime<chrono::Utc>) -> String {
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

fn main() {
    let config = load_config();
    let now_utc = chrono::Utc::now();
    let local_tz: Tz = iana_time_zone::get_timezone()
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(chrono_tz::UTC);

    if config.clocks.is_empty() {
        println!("No clocks configured. Edit ~/.config/fuso/clocks.json");
        return;
    }

    // Calculate column widths
    let max_name = config.clocks.iter().map(|c| c.name.len()).max().unwrap_or(0);
    let max_city = config.clocks.iter().map(|c| c.city.len()).max().unwrap_or(0);

    println!();
    for entry in &config.clocks {
        let tz: Tz = match entry.timezone.parse() {
            Ok(tz) => tz,
            Err(_) => {
                eprintln!("  invalid timezone: {}", entry.timezone);
                continue;
            }
        };

        let now_tz = now_utc.with_timezone(&tz);
        let flag = entry.flag.as_deref().unwrap_or_else(|| timezone_to_flag(&entry.timezone));
        let time = now_tz.format("%H:%M").to_string();
        let day = now_tz.format("%a").to_string();
        let offset = relative_offset(local_tz, tz, now_utc);

        let status = current_availability(entry, now_tz)
            .map(|a| match a {
                Availability::Busy(label) => format!("  [{}]", label),
                Availability::Available => "  [available]".into(),
                Availability::DayOff => "  [day off]".into(),
            })
            .unwrap_or_default();

        println!(
            "  {} {:<width_name$}  {:<width_city$}  {}  {} {:>6}{}",
            flag,
            entry.name,
            entry.city,
            time,
            day,
            offset,
            status,
            width_name = max_name,
            width_city = max_city,
        );
    }
    println!();
}
