defmodule WebMazeWeb.QueryController do
  use WebMazeWeb, :controller

  alias WebMaze.Queries
  alias WebMaze.Queries.Query

  action_fallback WebMazeWeb.FallbackController

  def run(conn, %{"id" => id}) do
    {:ok, run} = Queries.create_run(%{name: id})
    # TODO: Run the NetMaze client.
    render(conn, "run.json", run: run)
  end

  def index(conn, %{"run" => run_id} = params) do
    {limit, start} = limit_start(params)
    queries = Queries.get_run!(run_id) |> Queries.queries_for_run()
    # TODO: Pagination.

    render(conn, "query_for_run.json",
      run_id: run_id,
      limit: limit,
      start: start,
      queries: queries
    )
  end

  def index(conn, _params) do
    queries = Queries.list_queries()
    render(conn, "index.json", queries: queries)
  end

  def create(conn, %{"query" => query_params}) do
    with {:ok, %Query{} = query} <- Queries.create_query(query_params) do
      conn
      |> put_status(:created)
      |> put_resp_header("location", Routes.query_path(conn, :show, query))
      |> render("show.json", query: query)
    end
  end

  def show(conn, %{"id" => id}) do
    query = Queries.get_query!(id)
    render(conn, "show.json", query: query)
  end

  def update(conn, %{"id" => id, "query" => query_params}) do
    query = Queries.get_query!(id)

    with {:ok, %Query{} = query} <- Queries.update_query(query, query_params) do
      render(conn, "show.json", query: query)
    end
  end

  def delete(conn, %{"id" => id}) do
    query = Queries.get_query!(id)

    with {:ok, %Query{}} <- Queries.delete_query(query) do
      send_resp(conn, :no_content, "")
    end
  end

  defp limit_start(params) do
    limit = min(String.to_integer(params["limit"] || "30"), 30)
    start = String.to_integer(params["start"] || "1")
    {limit, start}
  end
end
