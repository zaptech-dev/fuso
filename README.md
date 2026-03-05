# Fuso

A lightweight macOS menu bar app for tracking your team's timezones. Native Swift, no Electron, no dependencies.

Fuso sits in your menu bar and shows the current time for everyone you work with. Optionally, it can track availability based on rotating schedules — useful for teams with members who have factory shifts, hospital rotations, or any recurring time blocks.

Named after *fuso horario*, Portuguese for timezone.

## Install

### Homebrew

```bash
brew tap zaptech-dev/tap
brew install --cask fuso
```

### From source

```bash
git clone https://github.com/zaptech-dev/fuso.git
cd fuso
./install.sh
open /Applications/Fuso.app
```

Requires macOS 13+ and Swift 5.9+.

## Configuration

Fuso reads from `~/.config/fuso/clocks.json`. A default config is created on first launch.

```json
{
  "clocks": [
    {
      "name": "Me",
      "city": "New York",
      "timezone": "America/New_York"
    },
    {
      "name": "Yuki",
      "city": "Tokyo",
      "timezone": "Asia/Tokyo"
    },
    {
      "name": "Carlos",
      "city": "Sao Paulo",
      "timezone": "America/Sao_Paulo",
      "flag": "🇧🇷"
    }
  ]
}
```

Each clock entry has:

| Field | Required | Description |
|-------|----------|-------------|
| `name` | yes | Person or label |
| `city` | yes | Location description |
| `timezone` | yes | IANA timezone identifier (e.g. `Asia/Tokyo`) |
| `flag` | no | Override the auto-detected flag emoji |
| `status` | no | Availability schedule (see below) |

Fuso auto-detects country flags from timezone identifiers. You can override with the `flag` field.

The config is hot-reloaded every 10 seconds — no restart needed for config changes.

## Availability Status

For team members with rotating schedules, you can add a `status` block to show whether they're currently busy or available.

```json
{
  "name": "Sarah",
  "city": "London",
  "timezone": "Europe/London",
  "status": {
    "blocks": {
      "1": { "label": "Hospital", "start": "07:00", "end": "15:00" },
      "2": { "label": "Hospital", "start": "15:00", "end": "23:00" },
      "3": { "label": "Hospital", "start": "23:00", "end": "07:00" }
    },
    "months": {
      "2026-03": "0111110022222003333300111110022"
    }
  }
}
```

**blocks** defines named time blocks with a label and start/end times in the person's local timezone. The label is what gets displayed — "Hospital", "Office", "On call", "Class", whatever fits.

**months** maps each month to a string of digits, one per day. `0` means day off, any other digit references a block key. March has 31 days, so the string has 31 characters.

Cross-midnight shifts are handled automatically (e.g. a shift from 23:00 to 07:00).

The clock card shows a colored indicator:
- **Orange** — currently in a scheduled block (shows the label)
- **Green** — scheduled workday but not in a block right now
- **Gray** — day off

### Workflow

Your team member sends you their schedule as a JSON block. You paste it into their entry in `clocks.json`. When the next month comes, they send a new month string and you add it. Old months can stay or be cleaned up.

## Adding Clocks

You can add clocks directly from the UI (click the + button) or by editing the config file. Click the Config button in the app to open the JSON file in your default editor.

## License

MIT
