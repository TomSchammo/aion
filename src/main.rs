use clap::Parser;
use notify_rust::Notification;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    #[clap(short, long, help = "TBA")]
    time: String,
}

/// format: <a>d<b>h<c>m<e>s, where a,b,c,e are numbers
fn parse_time(time: &str) -> u64 {
    // total seconds
    let mut seconds: u64 = 0;

    // current number that is parsed
    let mut number: String = String::default();

    // true if the function expects the next
    // character to be a number. False otherwise.
    let mut should_be_number = true;

    for c in time.chars() {
        if c.is_numeric() {
            if should_be_number {
                should_be_number = false;
                number = String::default();
            }
            number.push(c);
        } else if should_be_number {
            unimplemented!("should not happen");
        } else {
            should_be_number = true;
            let factor: u64 = match c {
                'd' => {
                    #[cfg(debug_assertions)]
                    println!("{number} days");

                    86_400
                }

                'h' => {
                    #[cfg(debug_assertions)]
                    println!("{number} hours");

                    3_600
                }
                'm' => {
                    #[cfg(debug_assertions)]
                    println!("{number} minutes");

                    60
                }
                's' => {
                    #[cfg(debug_assertions)]
                    println!("{number} seconds");

                    1
                }
                _ => {
                    unimplemented!("todo invalid character")
                }
            };
            seconds += number.parse::<u64>().unwrap() * factor;
        }
    }

    seconds
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
