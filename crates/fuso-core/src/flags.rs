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
