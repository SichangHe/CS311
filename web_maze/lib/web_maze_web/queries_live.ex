defmodule WebMazeWeb.QueriesLive do
  use WebMazeWeb, :live_view

  alias WebMaze.Queries

  def mount(_params, _session, socket) do
    {:ok,
     assign(socket,
       run_id: "",
       queries: [],
       limit: 30,
       start: 1
     )}
  end

  def render(assigns) do
    ~H"""
    <form phx-submit="queries">
    <label for="run_id">Run ID:</label>
    <input type="text" name="run_id" id="run_id" {[value: @run_id]}>
    <label for="limit">Limit:</label>
    <input type="number" name="limit" id="limit" min="1" max="30" {[value: @limit]}>
    <label for="start">Start:</label>
    <input type="number" name="start" id="start" min="1" {[value: @start]}>
    <button type="submit">Search</button>
    </form>
    <ol>
    <%= for query <- Enum.slice(@queries, @start - 1, @limit) do %><li>
    Query <% query.id %> from <%= query.connection_source %> with port <%= query.connection_port %> to target port <%= query.query_target %>.
    </li><% end %>
    </ol>
    """
  end

  def handle_event("queries", %{"run_id" => run_id, "limit" => limit, "start" => start}, socket) do
    limit = String.to_integer(limit)
    start = String.to_integer(start)

    if run_id == socket.assigns[:run_id] do
      {:noreply, assign(socket, limit: limit, start: start)}
    else
      run = Queries.get_run!(run_id)

      case run.finished do
        nil ->
          {:noreply, put_flash(socket, :error, "Run #{run_id} has not finished.")}

        _ ->
          queries = Queries.queries_for_run(run)

          {:noreply, assign(socket, run_id: run_id, limit: limit, start: start, queries: queries)}
      end
    end
  rescue
    ArgumentError ->
      {:noreply, put_flash(socket, :error, "Invalid parameters.")}

    Ecto.NoResultsError ->
      {:noreply, put_flash(socket, :error, "Run #{run_id} not found.")}
  end
end
