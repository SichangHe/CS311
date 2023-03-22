defmodule NetMaze do
  @moduledoc """
  Documentation for `NetMaze`.
  """

  use Application

  @impl true
  def start(_type, args) do
    NetMaze.GenServer.start_link(args)
  end
end
