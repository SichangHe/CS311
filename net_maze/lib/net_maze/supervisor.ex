defmodule NetMaze.Supervisor do
  use Supervisor

  @impl true
  def init(opts) do
    NetMaze.GenServer.start_link(opts)
  end
end
