defmodule NetMaze do
  @moduledoc """
  Documentation for `NetMaze`.
  """

  use Application

  @impl true
  def start(_type, _args) do
    Supervisor.start_link(NetMaze.Supervisor, [])
  end
end
