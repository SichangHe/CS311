use clap::{arg, command, Parser};
use log::debug;
use web_maze_client::{request::request, response, ResultDyn};

fn main() -> ResultDyn<()> {
    env_logger::init();
    let args = Args::parse();
    debug!("{args:#?}");
    match args.cmd {
        Act::Submit { id } => {
            submit(&args.url, &id)?;
        }
        Act::List { limit, start } => {
            list(&args.url, limit, start)?;
        }
        Act::Stats => todo!(),
    }
    Ok(())
}

fn submit(host: &str, id: &str) -> ResultDyn<response::Submit> {
    let body = request(host, &format!("/api/run/{id}"), "POST")?;
    let submit_response = serde_json::from_str(&body)?;

    println!("\n{submit_response:#?}");
    Ok(submit_response)
}

fn list(host: &str, limit: Option<usize>, start: Option<usize>) -> ResultDyn<response::List> {
    let path = "/api/list".to_owned();
    let path = match (limit, start) {
        (None, None) => path,
        (Some(limit), None) => format!("{path}?limit={limit}"),
        (None, Some(start)) => format!("{path}?start={start}"),
        (Some(limit), Some(start)) => format!("{path}?limit={limit}&start={start}"),
    };
    let body = request(host, &path, "GET")?;

    let list_response = serde_json::from_str(&body)?;
    println!("\n{list_response:#?}");
    Ok(list_response)
}

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    cmd: Act,

    #[arg(short, long)]
    url: String,
}

#[derive(Parser, Debug)]
enum Act {
    Submit {
        #[arg(short, long)]
        id: String,
    },
    List {
        #[arg(short, long)]
        limit: Option<usize>,

        #[arg(short, long)]
        start: Option<usize>,
    },
    Stats,
}
