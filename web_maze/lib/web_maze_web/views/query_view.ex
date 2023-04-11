defmodule WebMazeWeb.QueryView do
  use WebMazeWeb, :view
  alias WebMazeWeb.QueryView

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
      query_target: query.query_target
    }
  end
end
