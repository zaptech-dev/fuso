mod config;

use config::{current_availability, load_config, local_tz, relative_offset, timezone_to_flag, Availability};
use chrono_tz::Tz;
use glib::ControlFlow;
use gtk4::prelude::*;
use gtk4::{
    gdk, glib, Align, Application, ApplicationWindow, Box as GtkBox, CssProvider, Label,
    Orientation, ScrolledWindow,
};

const APP_ID: &str = "dev.zaptech.fuso";
const CSS: &str = r#"
    window {
        background-color: #ffffff;
    }
    .header-title {
        font-size: 18px;
        font-weight: bold;
        color: #1a1a1a;
    }
    .header-subtitle {
        font-size: 12px;
        color: #888888;
    }
    .clock-card {
        background-color: #f5f5f5;
        border-radius: 8px;
        padding: 10px 12px;
    }
    .clock-name {
        font-size: 13px;
        font-weight: 600;
        color: #1a1a1a;
    }
    .clock-city {
        font-size: 11px;
        color: #888888;
    }
    .clock-time {
        font-size: 20px;
        font-weight: bold;
        color: #1a1a1a;
    }
    .clock-meta {
        font-size: 10px;
        color: #888888;
    }
    .clock-flag {
        font-size: 22px;
    }
    .status-busy {
        font-size: 10px;
        color: #e67e22;
    }
    .status-available {
        font-size: 10px;
        color: #27ae60;
    }
    .status-dayoff {
        font-size: 10px;
        color: #888888;
    }
    .bottom-bar {
        padding: 10px 16px;
        border-top: 1px solid #e0e0e0;
    }
    .bottom-button {
        font-size: 12px;
        color: #888888;
        background: none;
        border: none;
        box-shadow: none;
        padding: 4px 8px;
    }
    .bottom-button:hover {
        color: #1a1a1a;
    }
"#;

fn build_clock_card(entry: &config::ClockEntry, now_utc: chrono::DateTime<chrono::Utc>, local_tz: Tz) -> GtkBox {
    let card = GtkBox::new(Orientation::Horizontal, 0);
    card.add_css_class("clock-card");

    let tz: Tz = entry.timezone.parse().unwrap_or(chrono_tz::UTC);
    let now_tz = now_utc.with_timezone(&tz);

    // Left side: flag + name/city/status
    let left = GtkBox::new(Orientation::Horizontal, 10);
    left.set_hexpand(true);

    let flag_text = entry.flag.as_deref().unwrap_or_else(|| timezone_to_flag(&entry.timezone));
    let flag = Label::new(Some(flag_text));
    flag.add_css_class("clock-flag");
    flag.set_valign(Align::Center);
    left.append(&flag);

    let info = GtkBox::new(Orientation::Vertical, 1);

    let name = Label::new(Some(&entry.name));
    name.add_css_class("clock-name");
    name.set_halign(Align::Start);
    info.append(&name);

    let city = Label::new(Some(&entry.city));
    city.add_css_class("clock-city");
    city.set_halign(Align::Start);
    info.append(&city);

    if let Some(avail) = current_availability(entry, now_tz) {
        let (text, class) = match avail {
            Availability::Busy(ref label) => (label.clone(), "status-busy"),
            Availability::Available => ("Available".into(), "status-available"),
            Availability::DayOff => ("Day off".into(), "status-dayoff"),
        };
        let status = Label::new(Some(&format!("\u{25cf} {}", text)));
        status.add_css_class(class);
        status.set_halign(Align::Start);
        info.append(&status);
    }

    left.append(&info);
    card.append(&left);

    // Right side: time + day/offset
    let right = GtkBox::new(Orientation::Vertical, 1);
    right.set_valign(Align::Center);

    let time_str = now_tz.format("%H:%M").to_string();
    let time = Label::new(Some(&time_str));
    time.add_css_class("clock-time");
    time.set_halign(Align::End);
    right.append(&time);

    let day_str = now_tz.format("%a").to_string();
    let offset_str = relative_offset(local_tz, tz, now_utc);
    let meta = Label::new(Some(&format!("{} \u{00b7} {}", day_str, offset_str)));
    meta.add_css_class("clock-meta");
    meta.set_halign(Align::End);
    right.append(&meta);

    card.append(&right);
    card
}

fn build_ui(app: &Application) {
    let main_box = GtkBox::new(Orientation::Vertical, 0);

    // Header
    let header = GtkBox::new(Orientation::Vertical, 2);
    header.set_margin_start(16);
    header.set_margin_end(16);
    header.set_margin_top(16);
    header.set_margin_bottom(12);

    let title = Label::new(Some("Fuso"));
    title.add_css_class("header-title");
    title.set_halign(Align::Start);
    header.append(&title);

    let subtitle = Label::new(None);
    subtitle.add_css_class("header-subtitle");
    subtitle.set_halign(Align::Start);
    header.append(&subtitle);
    main_box.append(&header);

    // Scrollable clock list
    let scroll = ScrolledWindow::new();
    scroll.set_vexpand(true);
    scroll.set_max_content_height(420);
    scroll.set_propagate_natural_height(true);

    let list_box = GtkBox::new(Orientation::Vertical, 6);
    list_box.set_margin_start(12);
    list_box.set_margin_end(12);
    list_box.set_margin_bottom(8);
    scroll.set_child(Some(&list_box));
    main_box.append(&scroll);

    // Bottom bar
    let bottom = GtkBox::new(Orientation::Horizontal, 16);
    bottom.add_css_class("bottom-bar");

    let config_btn = gtk4::Button::with_label("Config");
    config_btn.add_css_class("bottom-button");
    config_btn.connect_clicked(|_| {
        let path = config::config_path();
        let _ = std::process::Command::new("xdg-open").arg(path).spawn();
    });
    bottom.append(&config_btn);

    let spacer = GtkBox::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    bottom.append(&spacer);

    let quit_btn = gtk4::Button::with_label("Quit");
    quit_btn.add_css_class("bottom-button");
    quit_btn.connect_clicked(|btn| {
        if let Some(window) = btn.root().and_then(|r| r.downcast::<ApplicationWindow>().ok()) {
            window.close();
        }
    });
    bottom.append(&quit_btn);
    main_box.append(&bottom);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Fuso")
        .default_width(320)
        .resizable(false)
        .child(&main_box)
        .build();

    // Initial render
    refresh_clocks(&list_box, &subtitle);

    // Auto-refresh every second
    let list_ref = list_box.clone();
    let sub_ref = subtitle.clone();
    let mut tick_count: u32 = 0;
    glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
        tick_count += 1;
        refresh_clocks(&list_ref, &sub_ref);
        // Reload config every 10 seconds
        if tick_count % 10 == 0 {
            refresh_clocks(&list_ref, &sub_ref);
        }
        ControlFlow::Continue
    });

    window.present();
}

fn refresh_clocks(list_box: &GtkBox, subtitle: &Label) {
    // Remove existing children
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    let config = load_config();
    let now_utc = chrono::Utc::now();
    let ltz = local_tz();

    subtitle.set_text(&format!("{} clocks", config.clocks.len()));

    for entry in &config.clocks {
        let card = build_clock_card(entry, now_utc, ltz);
        list_box.append(&card);
    }
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| {
        let provider = CssProvider::new();
        provider.load_from_string(CSS);
        gtk4::style_context_add_provider_for_display(
            &gdk::Display::default().expect("could not get display"),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    app.connect_activate(build_ui);
    app.run();
}
