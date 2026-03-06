use chrono_tz::Tz;
use fuso_core::{
    current_availability, load_config, local_tz, relative_offset, timezone_to_flag, Availability,
};

fn main() {
    let config = load_config();
    let now_utc = chrono::Utc::now();
    let ltz = local_tz();

    if config.clocks.is_empty() {
        println!("No clocks configured. Edit ~/.config/fuso/clocks.json");
        return;
    }

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
        let offset = relative_offset(ltz, tz, now_utc);

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
