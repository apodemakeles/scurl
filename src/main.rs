use std::str::FromStr;
use std::string::ParseError;
use clap:: {Subcommand, Parser};
use reqwest::{Client, header, Response, Url};
use anyhow::Result;
use colored::*;
use mime::Mime;

/// a super curl
#[derive(Parser, Debug)]
#[clap()]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,

    /// print full message
    #[clap(short)]
    verbose: bool,
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
    // println!("{:?}", opts);

    let result = match opts.subcmd {
        SubCommand::Get(ref cmd) => execute_get(cmd, &opts).await?,
    };

    Ok(result)
}

async fn execute_get(cmd: &GetCmd, opts: &Opts) -> Result<()>{
    let client = Client::new();
    let resp = client.get(&cmd.url.to_string()).send().await?;
    Ok(print_resp(resp, opts).await?)
}

fn print_status(resp: &Response){
    let status_line = format!("{:?}, {}", resp.version(), resp.status()).blue();
    println!("{}\n", status_line);
}

fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }

    print!("\n");
}

fn get_content_type(resp: &Response) -> Option<Mime>{
    resp.headers().get(header::CONTENT_TYPE).
        map(|v|v.to_str().unwrap().parse().unwrap())
}

fn print_body(m: Option<Mime>, body: &String) {
    match m{
        Some(v) => if v == mime::APPLICATION_JSON{
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        },
        _ => println!("{}", body),
    }
}

async fn print_resp(resp: Response, opts: &Opts)-> Result<()>{
    if opts.verbose{
        print_status(&resp);
        print_headers(&resp);
    }
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);

    Ok(())
}
