use clap::{arg, command, Parser};
use log::debug;

use web_maze_client::{
    request::{list, queries, submit},
    stats::stats,
    ResultDyn,
};

fn main() -> ResultDyn<()> {
    env_logger::init();
    let args = Args::parse();
    debug!("{args:#?}");
    match args.cmd {
        Act::Submit { id } => {
            let submit_response = submit(&args.url, &id)?;
            println!("{submit_response:#?}");
        }
        Act::List { limit, start } => {
            let list_response = list(&args.url, limit, start)?;
            println!("{list_response:#?}");
        }
        Act::Queries {
            run_id,
            limit,
            start,
        } => {
            let queries = queries(&args.url, &run_id, limit, start)?;
            println!("{queries:#?}");
        }
        Act::Stats => {
            let stats = stats(&args.url)?;
            println!("{stats:#?}");
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    cmd: Act,

    /// Host server URL
    #[arg(short, long)]
    url: String,
}

#[derive(Parser, Debug)]
enum Act {
    /// Submit a new run
    Submit {
        #[arg(short, long)]
        id: String,
    },
    /// List finished runs
    List {
        #[arg(short, long)]
        limit: Option<usize>,

        #[arg(short, long)]
        start: Option<usize>,
    },
    /// Queries of a run
    Queries {
        #[arg(short, long)]
        run_id: String,

        #[arg(short, long)]
        limit: Option<usize>,

        #[arg(short, long)]
        start: Option<usize>,
    },
    /// Mean and variance of numbers of queries of all finished runs
    Stats,
}
