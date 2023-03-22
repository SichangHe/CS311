defmodule NetMaze do
  @moduledoc """
  Documentation for `NetMaze`.
  """

  use Application

  @impl true
  def start(_type, args) do
    Supervisor.start_link([{NetMaze.GenServer, args}], strategy: :one_for_all)
  end
end
