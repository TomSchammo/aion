use std::{thread, time};

use clap::Parser;
use notify_rust::Notification;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    #[clap(short = 's', long = "set", help = "TBA")]
    time: String,
    #[clap(
        short = 'i',
        long = "interactive",
        takes_value = false,
        help = "Keeps aion in the foreground\nTBA: cli"
    )]
    interactive: bool,
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
        .body(format!("Your timer of {} has ended!", time).as_str())
        .icon(icon)
        .appname("aion")
        .timeout(0)
        .show()
        .unwrap();
}

fn main() {
    let args = Args::parse();

    let daemon = aion::prepare_daemon();

    if args.interactive {
        let time = aion::parse_time(&args.time);
        thread::sleep(time::Duration::from_secs(time));
        display_notification(args.time.as_str(), None);
    } else {
        match daemon.start() {
            Ok(_) => {
                println!("Successfully started daemon!");

                let time = aion::parse_time(&args.time);
                thread::sleep(time::Duration::from_secs(time));
                display_notification(args.time.as_str(), None);
            }
            Err(e) => {
                eprintln!("Error when starting deamon!");
                eprintln!("{e}");
            }
        }
    }
}
