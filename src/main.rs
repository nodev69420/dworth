use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'r')]
    run: String,
}

fn main() -> Result<(), ()> {
    let args = Args::parse();

    dworth::run(args.run.as_str())
}
