# Web Maze Client

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install).

## Run

In this directory, run `cargo r --` with the arguments.

Results are deserialized into Rust structs for readability.

### Help

```shell
$ cargo r -- help
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/web_maze_client help`
Usage: web_maze_client --url <URL> <COMMAND>

Commands:
  submit   Submit a new run
  list     List finished runs
  queries  Queries of a run
  stats    Mean and variance of numbers of queries of all finished runs
  help     Print this message or the help of the given subcommand(s)

Options:
  -u, --url <URL>  Host server URL
  -h, --help       Print help
  -V, --version    Print version
```

Get help for a command:

```shell
$ cargo r -- help list
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/web_maze_client help list`
List finished runs

Usage: web_maze_client --url <URL> list [OPTIONS]

Options:
  -l, --limit <LIMIT>
  -s, --start <START>
  -h, --help           Print help
```

### Submit

```shell
$ cargo r -- -u 127.0.0.1:4000 submit -i myId
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/web_maze_client -u '127.0.0.1:4000' submit -i myId`
Submit {
    run_id: "15",
}
```

### List

```shell
$ cargo r -- -u 127.0.0.1:4000 list -l2 -s2
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/web_maze_client -u '127.0.0.1:4000' list -l2 -s2`
List {
    limit: 2,
    start: 2,
    run_ids: [
        "9",
        "10",
    ],
    prev: Some(
        "/api/list?limit=1&start=1",
    ),
    next: Some(
        "/api/list?&limit=2&start=4",
    ),
}
```

### Stats

```shell
$ cargo r -- -u 127.0.0.1:4000 stats
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/web_maze_client -u '127.0.0.1:4000' stats`
Stats {
    mean: 79.125,
    variance: 869.609375,
}
```

To validate the results are correct, use `WebMaze.Client.mean_variance` from
the server as specified in Test client `stats` command in `web_maze/README.md`.

## Debug

To show debug information during the run, set the environment variable
`RUST_LOG`:

```shell
export RUST_LOG=debug
```

To only show debug information of certain modules, e.g. `web_maze_client::stats`:

```shell
export RUST_LOG=web_maze_client::stats=debug
```

The other module with debug information is `web_maze_client::request`.

## Use as a standalone binary

Compile in release mode.

```shell
cargo b --release
```

The binary is at `target/release/web_maze_client`.
