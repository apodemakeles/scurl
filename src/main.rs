use std::str::FromStr;
use std::string::ParseError;
use clap:: {Subcommand, Parser};
use reqwest::Url;

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
    #[clap(parse(try_from_str = parse_to_url))]
    url: Url,
}

fn parse_to_url(s: &str) -> Result<Url, String> {
    Url::from_str(s).map_err(|_| {
        format!("{} is not a legal url ", s)
    })
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts)
}
