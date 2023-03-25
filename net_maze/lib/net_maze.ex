defmodule NetMaze do
  @moduledoc """
  `NetMaze` is an application that tries to solve the NetMaze game.
  """

  use Application
  require Logger

  @impl true
  def start(_type, args) do
    {:ok, _} =
      Supervisor.start_link(
        [
          {DynamicSupervisor,
           name: NetMaze.Supervisor, strategy: :one_for_one, max_children: :infinity}
        ],
        strategy: :one_for_one
      )

    loop(args)
  end

  def loop(args) do
    try do
      DynamicSupervisor.start_child(
        NetMaze.Supervisor,
        {NetMaze.GenServer, Keyword.put(args, :message, rand_str())}
      )
    catch
      kind, _value when kind in [:exit, :throw] ->
        :timer.sleep(500)
    end

    loop(args)
  end

  def rand_str() do
    len = 8..24 |> Enum.random()
    ?a..?z |> Enum.take_random(len) |> List.to_string()
  end
end
