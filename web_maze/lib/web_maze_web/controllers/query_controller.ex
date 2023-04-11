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
end
