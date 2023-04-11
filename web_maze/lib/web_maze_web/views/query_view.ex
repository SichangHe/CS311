defmodule WebMazeWeb.QueryView do
  use WebMazeWeb, :view
  alias WebMazeWeb.QueryView

  def render("query_for_run.json", %{run_id: run_id, limit: limit, start: start, queries: queries}) do
    %{
      runId: "#{run_id}",
      limit: limit,
      start: start,
      queries: render_many(queries, QueryView, "query.json")
    }
  end

  def render("run.json", %{run: run}) do
    %{runId: "#{run.id}"}
  end

  def render("index.json", %{queries: queries}) do
    %{data: render_many(queries, QueryView, "query.json")}
  end

  def render("show.json", %{query: query}) do
    %{data: render_one(query, QueryView, "query.json")}
  end

  def render("query.json", %{query: query}) do
    %{
      id: query.id,
      connection_source: query.connection_source,
      connection_port: query.connection_port,
      query_target: query.query_target,
      run_id: query.run_id
    }
  end
end
