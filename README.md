# WebMaze server and client

See the corresponding `README.md` for instructions.

## WebMaze server

- Under `web_maze/`.
- In Elixir with Phoenix Framework.
    - Ecto for ORM and SQLite3 for database.
    - Phoenix LiveView for web interface.
    - Phoenix Swagger for REST API documentation.

### Why Elixir and Phoenix

- Functional, easy to reason about.
    Important since web apps are hard to thoroughly test.
- High observability. Nice REPL, error message, and `dbg` macro.
- Managed full parallelism and concurrency. Great performance for free.
- Great scalability on top of the BEAM.
- LiveView is the simplest way to interactive web interface.

### Improvement WebMaze server could have

- Proper database level-pagination.
    Not supported because of a bug in Exqlite, the SQLite3 adapter for Ecto.
- Web interface styling.

### Challenges in WebMaze server

- Learning a new framework.
- Proper `has_many` and `belongs_to` schema and migration using Ecto.
- The Exqlite bug that prevents database-level pagination.
- Lack of documentation for Phoenix Swagger.

## WebMaze client

- Under `web_maze_client/`.
- In Rust.
    - `clap` for CLI.
    - `serde` for JSON response parsing.
    - `std::net::TcpStream` for TCP connection.
    - `lazy-regex` for URL parsing.
    - `env-logger` for logging configuration.

### Why Rust, `clap`, and `serde`

- Portable and fast start up with great toolchain. Great for CLI.
- `clap` offers fully declarative CLI arguments parsing.
- `serde` offers type-safe and worry-free JSON parsing.

### Improvement WebMaze client could have

- Parallel web request.
    The current version is entirely sequential for simplicity.

### Challenges in WebMaze client

- TCP connection stream reader hang after reading response headers.
    - Caused by the lack of `\n` in response body.
    - Solved by reading `content-length` bytes after headers.

## Acknowledgements

Some web pages I referred to:

- <https://doc.rust-lang.org/stable/book/ch20-01-single-threaded.html>
- <https://stackoverflow.com/questions/71478238/rust-tcpstream-reading-http-request-sometimes-lose-the-body>
- <https://docs.rs/clap/4.2.2/clap/_derive/_tutorial/index.html>
- <https://swagger.io/blog/api-development/generate-api-documentation-effortlessly-from-your/>
- <https://hexdocs.pm/phoenix_swagger/getting-started.html>
- <https://hexdocs.pm/phoenix_swagger/PhoenixSwagger.html#swagger_schema/1>

Additionally, I used Bing Chat to

- Generate the initial implementation for pagination and fix off-by-one error
    created by pages starting from 1.
- Get reference implementation to calculate mean and variance in both Rust
    and Elixir.
    - I was looking for functions in the standard libraries,
        but was offered low-level implementations.
- Attempt to generate `clap` code but it was using out-of-date `clap3`.
- Generate the first half of Phoenix Swagger code for documentation.
    - I thought Bing Chat could generate all the code I needed because it was
        just documentation labor.
    - It failed to generate correct response schema so I had to hunt down the
        Phoenix Swagger documentation and hand-write them.
- Generate the starter code for the first two LiveView pages.
    - I was new to LiveView and used Bing Chat to get some code to start.
