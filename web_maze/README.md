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

## Development

### Update Swagger file

```shell
mix phx.swagger.generate
```
