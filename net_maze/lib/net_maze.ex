defmodule NetMaze do
  @moduledoc """
  `NetMaze` is an application that tries to solve the NetMaze game.
  """

  use Application

  @impl true
  def start(_type, args) do
    start = NetMaze.Supervisor.start_link(args, name: NetMaze.Supervisor)
    # Wait until NetMaze.GenServer finishes.
    reference = Process.monitor(NetMaze.GenServer)

    receive do
      {:DOWN, ^reference, _, _, _} -> IO.puts("NetMaze.GenServer finished.")
    end

    start
  end
end
