use std::str::FromStr;
use std::string::ParseError;
use clap:: {Subcommand, Parser};
use reqwest::{Client, Url};
use anyhow::Result;

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

#[tokio::main]
async fn main()-> Result<()> {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);

    let result = match opts.subcmd {
        SubCommand::Get(ref cmd) => execute_get(cmd, &opts).await?,
    };

    Ok(result)
}

async fn execute_get(cmd: &GetCmd, opts: &Opts) -> Result<()>{
    let client = Client::new();
    let resp = client.get(&cmd.url.to_string()).send().await?;
    println!("{:?}", resp.text().await?);

    Ok(())
}
