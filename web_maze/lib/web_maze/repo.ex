defmodule WebMaze.Repo do
  use Ecto.Repo,
    otp_app: :web_maze,
    adapter: Ecto.Adapters.SQLite3
end
