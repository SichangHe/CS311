use clap::{arg, command, Parser};
use log::debug;

use web_maze_client::{
    request::{list, queries, submit},
    ResultDyn,
};

fn main() -> ResultDyn<()> {
    env_logger::init();
    let args = Args::parse();
    debug!("{args:#?}");
    match args.cmd {
        Act::Submit { id } => {
            let submit_response = submit(&args.url, &id)?;
            println!("\n{submit_response:#?}");
        }
        Act::List { limit, start } => {
            let list_response = list(&args.url, limit, start)?;
            println!("\n{list_response:#?}");
        }
        Act::Queries {
            run_id,
            limit,
            start,
        } => {
            let queries = queries(&args.url, &run_id, limit, start)?;
            println!("\n{queries:#?}");
        }
        Act::Stats => todo!(),
    }
    Ok(())
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
    Queries {
        #[arg(short, long)]
        run_id: String,

        #[arg(short, long)]
        limit: Option<usize>,

        #[arg(short, long)]
        start: Option<usize>,
    },
    Stats,
}
