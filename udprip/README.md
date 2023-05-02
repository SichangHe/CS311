# UDPRIP: User Datagram Protocol Routing Information Protocol

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install).

## Run

In this directory, run `cargo r --` followed by the arguments.

Alternatively, compile first:

```sh
cargo b --release
```

and then add the directory to the binary to path:

```sh
set -gx PATH target/release/ PATH
```

and directly run:

```sh
udprip
```

### Arguments

```sh
$ udprip --help
Usage: udprip <ADDRESS> <PERIOD> [STARTUP]

Arguments:
  <ADDRESS>
  <PERIOD>
  [STARTUP]

Options:
  -h, --help  Print help
```

Inside the running shell:

```sh
> help
Usage: udprip <COMMAND>

Commands:
  add
  del
  trace
  quit
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

> help add
Usage: add <IP> <WEIGHT>

Arguments:
  <IP>
  <WEIGHT>

Options:
  -h, --help  Print help

> help del
Usage: del <IP>

Arguments:
  <IP>

Options:
  -h, --help  Print help

> help trace
Usage: trace <IP>

Arguments:
  <IP>

Options:
  -h, --help  Print help

> help help
Print this message or the help of the given subcommand(s)

Usage: help [COMMAND]...

Arguments:
  [COMMAND]...  Print help for the subcommand(s)

```

## Features

- Each component fully asynchronous and non-blocking.
- Help commands in the shell using Clap.
- Sane shell (up arrow history, keyboard navigation, etc.) using RustyLine.
- Load balancing: randomly select next hop if best routes have the same weight.

## Implementation details

### Periodic update

`route::manage` spawns function `route::time_notify` in a Tokio thread.
`route::time_notify` tries receiving messages that resets its timeout;
if the timeout happens before it gets any message,
it sends a `Route::Notify` to `route::manage`,
which then calls `route::notify` to notify the peers.

When a `add` or `del` command is triggered,
a notification is immediate scheduled.
When `route::notify` is called, it sends a message to `route::time_notify` to
reset its timeout.

Since `route::manage` could be blocking on other tasks when a `Route::Notify`
arrives,
the delay between each update could be slightly longer than the delay
configured,
but the difference should be within a microsecond for small networks.

### Tracking next routers

In `route::manage`,
`next` is used to track next routers:

```
next: BTreeMap<IpAddr, (usize, Vec<IpAddr>)>
               ^          ^         ^
        destination     weight     list of next routers
```

Every time `accept` or `peers` is updated,
`next` is recalculated by `route::calculate_next` using these variables.
`accept: BTreeMap<IpAddr, usize>` records the neighbor routers and their weights
that UDPRIP learned from `add` and `del` commands.
`peers: Peers` records the route information UDPRIP received from packets.

```rust
struct Peers {
    peers: BTreeMap<IpAddr, BTreeMap<IpAddr, usize>>,
    timeout_peers: BTreeMap<IpAddr, JoinHandle<()>>,
    peer_senders: BTreeMap<IpAddr, Sender<()>>,
}
```

`Peers` stores the routes received in `peers`:

```
peers: BTreeMap<IpAddr, BTreeMap<IpAddr, usize>>
                ^                 ^          ^
            source        destination       distance
```

### Removal of dead peer routes

In `timeout_peers`,
`Peers` stores Tokio threads running `route::timeout_peer` function:

```
timeout_peers: BTreeMap<IpAddr, JoinHandle<()>>
                        ^               ^
                    source    timeout_peer thread handle
```

`route::timeout_peer` tries receiving messages that resets its timeout;
if its timeout is reached, it sends the source `ip` in a `Route::DelPeer { ip }`
to `route::manage`,
which then calls `Peers::remove` and `route::calculate_next` to
remove `ip` from `Peers` and recalculate `next`.
If an update for `ip` comes and `ip` is in `peers`,
`Peer::insert` will send a message to the `route::timeout_peer` process
corresponding to `ip` to reset its timeout.

The delay between the strict timeout and the removal of dead peer routes could
be caused by two main factors:
`route::manage` blocking on other tasks before receiving `Route::DelPeer`;
and the Tokio runtime not triggering the timeout inside `route::timeout_peer`
on time.
Both factors should have negligible impact.

## Debug

To show debug information during the run, set the environment variable
`RUST_LOG`:

```sh
set -gx RUST_LOG udprip=debug
```

To only show debug information for certain modules,
specify them separated by `,`, for example:

```sh
set -gx RUST_LOG udprip::route=debug,udprip::socket=debug
```

Other modules include `udprip::command` and `udprip::message`.
