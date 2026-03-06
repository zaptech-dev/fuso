use chrono::Offset;
use chrono_tz::Tz;

pub fn relative_offset(local_tz: Tz, remote_tz: Tz, now: chrono::DateTime<chrono::Utc>) -> String {
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
