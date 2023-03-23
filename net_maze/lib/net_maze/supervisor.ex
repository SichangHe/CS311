defmodule NetMaze.Supervisor do
  use Supervisor

  @impl true
  def init(args) do
    [{NetMaze.GenServer, args}]
    |> Supervisor.init(strategy: :one_for_all)
  end

  @spec start_link(NetMaze.GenServer.args(), [{:name, atom | {:global, any} | {:via, atom, any}}]) ::
          {:ok, pid}
  def start_link(args, opts \\ []) do
    Supervisor.start_link(__MODULE__, args, opts)
  end
end
