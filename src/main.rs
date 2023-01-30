use clap::Parser;

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

fn main() {
    let args = Args::parse();

    let daemon = aion::prepare_daemon();

    if args.interactive {
        aion::run_timer(&args.time);
    } else {
        match daemon.start() {
            Ok(_) => {
                println!("Successfully started daemon!");

                aion::run_timer(&args.time);
            }
            Err(e) => {
                eprintln!("Error when starting deamon!");
                eprintln!("{e}");
            }
        }
    }
}
