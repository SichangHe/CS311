defmodule WebMazeWeb.RunLive do
  use WebMazeWeb, :live_view
  require Logger

  alias WebMaze.Queries

  def mount(_params, _session, socket) do
    {:ok, assign(socket, run: nil, finished: false, queries: [])}
  end

  def render(assigns) do
    ~H"""
    <form phx-submit="run">
    <label for="id">ID:</label>
    <input type="text" name="id" id="id">
    <button type="submit">Run</button>
    </form>
    <%= if @run do %>
    <div>Run ID: <%= @run.id %>.</div>
    <div><%= if @finished do %>
    Run finished.
    <% else %>
    Running…
    <% end %></div>
    <ol>
    <%= for query <- @queries do %><li>
    Query from <%= query.connection_source %> with port <%= query.connection_port %> to target port <%= query.query_target %>.
    </li><% end %>
    </ol>
    <% end %>
    """
  end

  def handle_info(:finished, socket) do
    {:noreply, assign(socket, finished: true)}
  end

  def handle_info({:query, query}, socket) do
    {:noreply, update(socket, :queries, &[query | &1])}
  end

  def handle_event(
        "run",
        %{"id" => id},
        socket
      ) do
    {:ok, run} = Queries.create_run(%{name: id})
    me = self()

    query_call = fn connection_source, connection_port, query_target ->
      case Queries.create_query(%{
             connection_source: connection_source,
             connection_port: connection_port,
             query_target: query_target,
             run_id: run.id
           }) do
        {:error, changeset} ->
          Logger.error("Error creating query with #{inspect(changeset)}")

        {:ok, query} ->
          send(me, {:query, query})
          :ok
      end
    end

    finish_call = fn ->
      Queries.finish_run(run)
      send(me, :finished)
    end

    args = [
      ip: System.get_env("IP") || "67.159.95.167",
      port: System.get_env("PORT") || 51300,
      message: id,
      query_call: query_call,
      finish_call: finish_call
    ]

    DynamicSupervisor.start_child(NetMaze.Supervisor, {NetMaze.GenServer, args})

    {:noreply, assign(socket, run: run)}
  end
end
