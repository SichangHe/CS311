defmodule NetMaze.Supervisor do
  use Supervisor

  @impl true
  def init(_opts) do
    [NetMaze.GenServer]
    |> Supervisor.init(strategy: :one_for_all)
  end
end
