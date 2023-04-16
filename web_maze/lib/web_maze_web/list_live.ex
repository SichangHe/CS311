defmodule WebMazeWeb.ListLive do
  use WebMazeWeb, :live_view

  alias WebMaze.Queries

  def mount(_params, _session, socket) do
    {:ok,
     assign(socket,
       runs:
         Queries.list_runs()
         |> Enum.filter(fn run -> run.finished != nil end),
       limit: 30,
       start: 1
     )}
  end

  def render(assigns) do
    ~H"""
    <form phx-submit="list">
    <label for="limit">Limit:</label>
    <input type="number" name="limit" id="limit" min="1" max="30" {[value: @limit]}>
    <label for="start">Start:</label>
    <input type="number" name="start" id="start" min="1" {[value: @start]}>
    <button type="submit">Update</button>
    </form>
    <ol>
    <%= for run <- Enum.slice(@runs, @start - 1, @limit) do %><li>
    Run <%= run.id %> of <%= run.name %> started at <%= run.inserted_at %> and finished at <%= run.finished %>
    </li><% end %>
    </ol>
    """
  end

  def handle_event("list", %{"limit" => limit, "start" => start}, socket) do
    limit = String.to_integer(limit)
    start = String.to_integer(start)
    {:noreply, assign(socket, limit: limit, start: start)}
  rescue
    ArgumentError ->
      {:noreply, put_flash(socket, :error, "Invalid parameters.")}
  end
end
