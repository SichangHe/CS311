defmodule NetMaze.GenServer do
  use GenServer

  @impl true
  def init(_opts) do
    IO.puts("Hi.")
    :ignore
  end

  def start_link(opts) do
    GenServer.start_link(__MODULE__, :ok, opts)
  end
end
