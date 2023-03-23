defmodule NetMaze do
  @moduledoc """
  `NetMaze` is an application that tries to solve the NetMaze game.
  """

  use Application

  @impl true
  def start(_type, args) do
    NetMaze.Supervisor.start_link(args, name: NetMaze.Supervisor)
  end
end
