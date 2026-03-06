import SwiftUI

class ClockViewModel: ObservableObject {
    @Published var clocks: [ClockEntry] = []
    @Published var now = Date()
    private var timer: Timer?
    private var tickCount = 0

    init() {
        reload()
        timer = Timer.scheduledTimer(withTimeInterval: 1, repeats: true) { [weak self] _ in
            guard let self = self else { return }
            DispatchQueue.main.async {
                self.now = Date()
                self.tickCount += 1
                if self.tickCount % 10 == 0 {
                    self.reload()
                }
            }
        }
    }

    func reload() {
        clocks = ConfigManager.shared.load().clocks
    }

    func removeClock(at index: Int) {
        guard index >= 0, index < clocks.count else { return }
        var config = ConfigManager.shared.load()
        config.clocks.remove(at: index)
        ConfigManager.shared.save(config)
        clocks = config.clocks
    }

    func addClock(_ entry: ClockEntry) {
        var config = ConfigManager.shared.load()
        config.clocks.append(entry)
        ConfigManager.shared.save(config)
        clocks = config.clocks
    }

    deinit { timer?.invalidate() }
}

struct ClockListView: View {
    @StateObject private var viewModel = ClockViewModel()
    @State private var showingAddForm = false
    @State private var newName = ""
    @State private var newCity = ""
    @State private var newTimezone = ""
    @State private var newFlag = ""
    @State private var addError: String?
    var onQuit: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            // Header
            VStack(alignment: .leading, spacing: 2) {
                Text("Fuso")
                    .font(.system(size: 18, weight: .bold))
                Text("\(viewModel.clocks.count) clocks")
                    .font(.system(size: 12))
                    .foregroundColor(.secondary)
            }
            .padding(.horizontal, 16)
            .padding(.top, 16)
            .padding(.bottom, 12)

            // Clock list
            ScrollView {
                VStack(spacing: 6) {
                    ForEach(Array(viewModel.clocks.enumerated()), id: \.offset) { index, entry in
                        ClockCard(entry: entry, now: viewModel.now)
                            .contextMenu {
                                Button("Remove \(entry.name)") {
                                    viewModel.removeClock(at: index)
                                }
                            }
                    }
                }
                .padding(.horizontal, 12)
            }
            .frame(maxHeight: 420)

            // Inline add form
            if showingAddForm {
                Divider().padding(.vertical, 8)

                VStack(alignment: .leading, spacing: 8) {
                    Text("New Clock")
                        .font(.system(size: 13, weight: .semibold))

                    HStack(spacing: 8) {
                        TextField("Name", text: $newName)
                            .textFieldStyle(.roundedBorder)
                        TextField("Flag", text: $newFlag)
                            .textFieldStyle(.roundedBorder)
                            .frame(width: 50)
                    }
                    TextField("City", text: $newCity)
                        .textFieldStyle(.roundedBorder)
                    TextField("Timezone (e.g. Asia/Tokyo)", text: $newTimezone)
                        .textFieldStyle(.roundedBorder)

                    if let error = addError {
                        Text(error)
                            .font(.system(size: 11))
                            .foregroundColor(.red)
                    }

                    HStack {
                        Spacer()
                        Button("Cancel") { resetAddForm() }
                        Button("Add") { submitAddForm() }
                            .buttonStyle(.borderedProminent)
                            .controlSize(.small)
                    }
                }
                .padding(.horizontal, 16)
                .font(.system(size: 12))
            }

            Divider().padding(.top, 8)

            // Bottom bar
            HStack(spacing: 16) {
                Button(action: {
                    withAnimation(.easeInOut(duration: 0.2)) {
                        if showingAddForm {
                            resetAddForm()
                        } else {
                            showingAddForm = true
                        }
                    }
                }) {
                    Label(showingAddForm ? "Cancel" : "Add", systemImage: showingAddForm ? "xmark" : "plus")
                }
                .buttonStyle(.plain)

                Button(action: {
                    let home = FileManager.default.homeDirectoryForCurrentUser
                    let f = home.appendingPathComponent(".config/fuso/clocks.json")
                    NSWorkspace.shared.open(f)
                }) {
                    Label("Config", systemImage: "doc.text")
                }
                .buttonStyle(.plain)

                Spacer()

                Button("Quit") { onQuit() }
                    .buttonStyle(.plain)
            }
            .font(.system(size: 12))
            .foregroundColor(.secondary)
            .padding(.horizontal, 16)
            .padding(.vertical, 10)
        }
        .frame(width: 320)
        .onAppear { viewModel.reload() }
    }

    private func resetAddForm() {
        showingAddForm = false
        newName = ""
        newCity = ""
        newTimezone = ""
        newFlag = ""
        addError = nil
    }

    private func submitAddForm() {
        let name = newName.trimmingCharacters(in: .whitespaces)
        let city = newCity.trimmingCharacters(in: .whitespaces)
        let tz = newTimezone.trimmingCharacters(in: .whitespaces)
        let flag = newFlag.trimmingCharacters(in: .whitespaces)

        guard !name.isEmpty, !city.isEmpty, !tz.isEmpty else {
            addError = "Name, city, and timezone are required."
            return
        }
        guard TimeZone(identifier: tz) != nil else {
            addError = "Invalid timezone identifier."
            return
        }

        viewModel.addClock(ClockEntry(
            name: name,
            city: city,
            timezone: tz,
            flag: flag.isEmpty ? nil : flag
        ))
        resetAddForm()
    }
}

struct ClockCard: View {
    let entry: ClockEntry
    let now: Date

    private var tz: TimeZone {
        TimeZone(identifier: entry.timezone) ?? .current
    }

    var body: some View {
        HStack(spacing: 0) {
            HStack(spacing: 10) {
                Text(entry.flagEmoji)
                    .font(.system(size: 22))

                VStack(alignment: .leading, spacing: 1) {
                    Text(entry.name)
                        .font(.system(size: 13, weight: .semibold))
                        .foregroundColor(.primary)
                    Text(entry.city)
                        .font(.system(size: 11))
                        .foregroundColor(.secondary)
                    if let availability = entry.currentAvailability(at: now) {
                        AvailabilityBadge(status: availability)
                    }
                }
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 1) {
                Text(timeString)
                    .font(.system(size: 20, weight: .bold, design: .rounded))
                    .foregroundColor(.primary)

                HStack(spacing: 4) {
                    Text(dayString)
                    Text("·")
                    Text(relativeOffset)
                }
                .font(.system(size: 10))
                .foregroundColor(.secondary)
            }
        }
        .padding(.horizontal, 12)
        .padding(.vertical, 10)
        .background(
            RoundedRectangle(cornerRadius: 8)
                .fill(Color(nsColor: .controlBackgroundColor))
        )
    }

    private var timeString: String {
        let fmt = DateFormatter()
        fmt.timeZone = tz
        fmt.dateFormat = "HH:mm"
        return fmt.string(from: now)
    }

    private var dayString: String {
        let fmt = DateFormatter()
        fmt.timeZone = tz
        fmt.dateFormat = "EEE"
        return fmt.string(from: now)
    }

    private var relativeOffset: String {
        let local = TimeZone.current.secondsFromGMT(for: now)
        let remote = tz.secondsFromGMT(for: now)
        let diff = remote - local
        let hours = diff / 3600
        let minutes = abs(diff % 3600) / 60

        if hours == 0 && minutes == 0 { return "local" }
        if minutes > 0 {
            return String(format: "%+d:%02dh", hours, minutes)
        }
        return String(format: "%+dh", hours)
    }
}

struct AvailabilityBadge: View {
    let status: AvailabilityStatus

    var body: some View {
        HStack(spacing: 4) {
            Circle()
                .fill(color)
                .frame(width: 6, height: 6)
            Text(label)
                .font(.system(size: 10))
                .foregroundColor(color)
        }
    }

    private var color: Color {
        switch status {
        case .busy: return .orange
        case .available: return .green
        case .dayOff: return .gray
        }
    }

    private var label: String {
        switch status {
        case .busy(let l): return l
        case .available: return "Available"
        case .dayOff: return "Day off"
        }
    }
}
