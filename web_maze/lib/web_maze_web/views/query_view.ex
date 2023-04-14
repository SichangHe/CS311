defmodule WebMazeWeb.QueryView do
  use WebMazeWeb, :view
  alias WebMazeWeb.QueryView

  def render("query_for_run.json", %{
        run_id: run_id,
        limit: limit,
        start: start,
        queries: queries,
        prev_start: prev_start,
        prev_limit: prev_limit,
        next_start: next_start,
        next_limit: next_limit
      }) do
    prev =
      case prev_limit do
        0 -> nil
        prev_limit -> "/api/queries?run=#{run_id}&limit=#{prev_limit}&start=#{prev_start}"
      end

    next =
      case next_limit do
        0 -> nil
        next_limit -> "/api/queries?run=#{run_id}&limit=#{next_limit}&start=#{next_start}"
      end

    %{
      runId: "#{run_id}",
      limit: limit,
      start: start,
      queries: render_many(queries, QueryView, "query.json"),
      prev: prev,
      next: next
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
