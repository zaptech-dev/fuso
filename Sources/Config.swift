import Foundation

struct StatusBlock: Codable {
    let label: String
    let start: String
    let end: String
}

struct StatusSchedule: Codable {
    let blocks: [String: StatusBlock]
    let months: [String: String]
}

enum AvailabilityStatus {
    case busy(label: String)
    case available
    case dayOff
}

struct ClockEntry: Codable, Identifiable {
    var id: String { "\(name)-\(timezone)" }
    let name: String
    let city: String
    let timezone: String
    var flag: String?
    var status: StatusSchedule?

    var flagEmoji: String {
        if let flag = flag, !flag.isEmpty { return flag }
        return Self.timezoneToFlag(timezone)
    }

    private static func timezoneToFlag(_ tz: String) -> String {
        let map: [String: String] = [
            "America/New_York": "US", "America/Chicago": "US",
            "America/Denver": "US", "America/Los_Angeles": "US",
            "America/Phoenix": "US", "America/Anchorage": "US",
            "Pacific/Honolulu": "US",
            "America/Sao_Paulo": "BR", "America/Campo_Grande": "BR",
            "America/Fortaleza": "BR", "America/Manaus": "BR",
            "America/Bahia": "BR", "America/Belem": "BR",
            "America/Recife": "BR", "America/Cuiaba": "BR",
            "America/Rio_Branco": "BR", "America/Porto_Velho": "BR",
            "America/Maceio": "BR", "America/Araguaina": "BR",
            "Asia/Tokyo": "JP",
            "Europe/London": "GB",
            "Europe/Paris": "FR", "Europe/Berlin": "DE",
            "Europe/Rome": "IT", "Europe/Madrid": "ES",
            "Europe/Lisbon": "PT", "Europe/Amsterdam": "NL",
            "Europe/Zurich": "CH", "Europe/Vienna": "AT",
            "Europe/Prague": "CZ", "Europe/Warsaw": "PL",
            "Europe/Stockholm": "SE", "Europe/Oslo": "NO",
            "Europe/Copenhagen": "DK", "Europe/Helsinki": "FI",
            "Europe/Dublin": "IE", "Europe/Moscow": "RU",
            "Asia/Shanghai": "CN", "Asia/Hong_Kong": "HK",
            "Asia/Seoul": "KR", "Asia/Singapore": "SG",
            "Asia/Kolkata": "IN", "Asia/Dubai": "AE",
            "Asia/Bangkok": "TH", "Asia/Jakarta": "ID",
            "Asia/Taipei": "TW",
            "Australia/Sydney": "AU", "Australia/Melbourne": "AU",
            "Australia/Perth": "AU", "Australia/Brisbane": "AU",
            "Pacific/Auckland": "NZ",
            "America/Toronto": "CA", "America/Vancouver": "CA",
            "America/Edmonton": "CA",
            "America/Mexico_City": "MX",
            "America/Argentina/Buenos_Aires": "AR",
            "America/Santiago": "CL", "America/Bogota": "CO",
            "America/Lima": "PE",
            "Africa/Johannesburg": "ZA", "Africa/Lagos": "NG",
            "Africa/Cairo": "EG", "Africa/Nairobi": "KE",
            "Asia/Riyadh": "SA", "Europe/Istanbul": "TR",
            "Asia/Jerusalem": "IL",
        ]
        guard let code = map[tz] else { return "🌍" }
        let base: UInt32 = 127397
        var result = ""
        for scalar in code.uppercased().unicodeScalars {
            if let u = UnicodeScalar(base + scalar.value) {
                result.append(String(u))
            }
        }
        return result.isEmpty ? "🌍" : result
    }
}

struct Config: Codable {
    var clocks: [ClockEntry]
}

class ConfigManager {
    static let shared = ConfigManager()

    private let configDir: URL
    private let configFile: URL

    private init() {
        let home = FileManager.default.homeDirectoryForCurrentUser
        configDir = home.appendingPathComponent(".config/fuso")
        configFile = configDir.appendingPathComponent("clocks.json")
    }

    func load() -> Config {
        guard FileManager.default.fileExists(atPath: configFile.path) else {
            let defaultConfig = Config(clocks: [
                ClockEntry(name: "Me", city: "Orlando", timezone: "America/New_York"),
            ])
            save(defaultConfig)
            return defaultConfig
        }

        do {
            let data = try Data(contentsOf: configFile)
            return try JSONDecoder().decode(Config.self, from: data)
        } catch {
            print("Failed to load config: \(error)")
            return Config(clocks: [])
        }
    }

    func save(_ config: Config) {
        do {
            try FileManager.default.createDirectory(at: configDir, withIntermediateDirectories: true)
            let encoder = JSONEncoder()
            encoder.outputFormatting = [.prettyPrinted, .sortedKeys]
            let data = try encoder.encode(config)
            try data.write(to: configFile)
        } catch {
            print("Failed to save config: \(error)")
        }
    }
}

extension ClockEntry {
    func currentAvailability(at date: Date) -> AvailabilityStatus? {
        guard let schedule = status else { return nil }

        let tz = TimeZone(identifier: timezone) ?? .current
        var cal = Calendar.current
        cal.timeZone = tz

        let fmt = DateFormatter()
        fmt.timeZone = tz
        fmt.dateFormat = "yyyy-MM"
        let monthKey = fmt.string(from: date)

        guard let monthStr = schedule.months[monthKey] else { return nil }

        let day = cal.component(.day, from: date)
        guard day >= 1, day <= monthStr.count else { return nil }

        let blockId = String(monthStr[monthStr.index(monthStr.startIndex, offsetBy: day - 1)])

        let hour = cal.component(.hour, from: date)
        let minute = cal.component(.minute, from: date)
        let nowMinutes = hour * 60 + minute

        // Check if currently in today's block
        if blockId != "0", let block = schedule.blocks[blockId] {
            let startMin = Self.parseTime(block.start)
            let endMin = Self.parseTime(block.end)

            if endMin > startMin {
                if nowMinutes >= startMin && nowMinutes < endMin {
                    return .busy(label: block.label)
                }
            } else if endMin < startMin {
                if nowMinutes >= startMin {
                    return .busy(label: block.label)
                }
            }
        }

        // Check if yesterday's block crosses midnight into today
        if let yesterday = cal.date(byAdding: .day, value: -1, to: date) {
            let yDay = cal.component(.day, from: yesterday)
            fmt.dateFormat = "yyyy-MM"
            let yMonthKey = fmt.string(from: yesterday)

            if let yMonthStr = schedule.months[yMonthKey],
               yDay >= 1, yDay <= yMonthStr.count {
                let yBlockId = String(yMonthStr[yMonthStr.index(yMonthStr.startIndex, offsetBy: yDay - 1)])

                if yBlockId != "0", let yBlock = schedule.blocks[yBlockId] {
                    let yStartMin = Self.parseTime(yBlock.start)
                    let yEndMin = Self.parseTime(yBlock.end)

                    if yEndMin < yStartMin && nowMinutes < yEndMin {
                        return .busy(label: yBlock.label)
                    }
                }
            }
        }

        return blockId == "0" ? .dayOff : .available
    }

    private static func parseTime(_ time: String) -> Int {
        let parts = time.split(separator: ":").compactMap { Int($0) }
        guard parts.count == 2 else { return 0 }
        return parts[0] * 60 + parts[1]
    }
}
