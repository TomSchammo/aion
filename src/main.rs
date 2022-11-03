use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    #[clap(short, long, help = "TBA")]
    time: String,
}

fn main() {
    let args = Args::parse();
}
