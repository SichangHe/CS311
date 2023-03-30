defmodule NetMaze do
  @moduledoc """
  `NetMaze` is an application that tries to solve the NetMaze game.
  """

  use Application
  require Logger

  @impl true
  def start(_type, args) do
    start = NetMaze.Supervisor.start_link(args, name: NetMaze.Supervisor)
    # Wait until NetMaze.GenServer finishes.
    GenServer.call(NetMaze.GenServer, :wait, 0xFFFF)
    start
  end
end
