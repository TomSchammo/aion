use clap::Parser;
use notify_rust::Notification;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    #[clap(short, long, help = "TBA")]
    time: String,
}


/// Sends a notification to the dbus that the specified time has elapsed.
///
/// Optionally an icon can be provided. If not the 'alarm-timer' icon
/// will be used as a fallback.
fn display_notification(time: &str, icon: Option<&str>) {
    let icon: &str = if let Some(icon) = icon {
        icon
    } else {
        "alarm-timer"
    };

    #[cfg(debug_assertions)]
    println!("Using icon: {}", icon);

    Notification::new()
        .summary("Time's up!")
        .body(format!("Your timer of {} has eneded!", time).as_str())
        .icon(icon)
        .show()
        .unwrap();
}

fn main() {
    let args = Args::parse();
}
