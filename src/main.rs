use clap:: { Subcommand, Parser, Args};

/// a super curl
#[derive(Parser, Debug)]
#[clap()]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
#[clap()]
enum SubCommand {
    Get(GetCmd),
}

#[derive(Parser, Debug)]
struct GetCmd{
    url: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts)
}
