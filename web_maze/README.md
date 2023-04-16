# WebMaze

## Prerequisites

- [Elixir](https://elixir-lang.org/install.html).
- [Phoenix setup](https://hexdocs.pm/phoenix/up_and_running.html).
    Note: you can skip the PostgreSQL part because we use SQLite3.

## Installation

- Install dependencies

    ```shell
    mix deps.get
    ```

- Create and migrate database

    ```shell
    mix ecto.setup
    ```

## Run

```shell
mix phx.server
```

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.

### Run configuration

You can set the IP and port the NetMaze client sends requests to by setting
the corresponding environment variables.

```shell
export IP=67.159.95.167
export PORT=51300
```

## Features

### Web interface equivalent to API

All the web interface pages are written using Phoenix LiveView.
The users are asked to fill in the parameters in the forms and submit them.

Submitting the forms do not trigger full reloads but instead update as needed.

- `/api/run/:id` => [`/run`](http://localhost:4000/run).

    The queries and the run status are live updated.
- `/api/queries` => [`/queries`](http://localhost:4000/queries).
- `/api/list` => [`/list`](http://localhost:4000/list).

### REST API documentation

See [`/api/doc/`](http://localhost:4000/api/doc/).

Generated using [phoenix_swagger](https://hexdocs.pm/phoenix_swagger/PhoenixSwagger.html).

Note that the web page might take 1min to load due to bugs from Swagger.

## Development

### Update Swagger file

```shell
mix phx.swagger.generate
```

### Test client `stats` command

Open an IEx session and call `WebMaze.Client.mean_variance`

```shell
$ iex -S mix
Erlang/OTP 25 [erts-13.2] [source] [64-bit] [smp:8:8] [ds:8:8:10] [async-threads:1] [jit] [dtrace]
Interactive Elixir (1.14.4) - press Ctrl+C to exit (type h() ENTER for help)
iex(1)> WebMaze.Client.mean_variance
```

The function returns a two-tuple, where the first element is the mean and
the second element is the variance.
