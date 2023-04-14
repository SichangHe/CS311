defmodule WebMazeWeb.QueryController do
  use WebMazeWeb, :controller

  alias WebMaze.Queries
  alias WebMaze.Queries.Query

  action_fallback WebMazeWeb.FallbackController

  def finished(conn, params) do
    case limit_start(params) do
      {:ok, {limit, start}} ->
        {run_ids, prev_start, prev_limit, next_start, next_limit} =
          Queries.list_runs()
          |> Enum.filter(fn run -> run.finished != nil end)
          |> Enum.map(fn run -> run.id end)
          |> paginate(start, limit)

        render(conn, "finished.json",
          limit: limit,
          start: start,
          run_ids: run_ids,
          prev_start: prev_start,
          prev_limit: prev_limit,
          next_start: next_start,
          next_limit: next_limit
        )

      :error ->
        send_resp(conn, :bad_request, "The limit must be 1 ~ 30!")
    end
  end

  def run(conn, %{"id" => id}) do
    {:ok, run} = Queries.create_run(%{name: id})
    # TODO: Run the NetMaze client.
    render(conn, "run.json", run: run)
  end

  def index(conn, %{"run" => run_id} = params) do
    case limit_start(params) do
      {:ok, {limit, start}} ->
        run = Queries.get_run!(run_id)

        case run.finished do
          nil ->
            # 204 No Content if run unfinished.
            send_resp(conn, :no_content, "")

          _ ->
            {queries, prev_start, prev_limit, next_start, next_limit} =
              run |> Queries.queries_for_run() |> paginate(start, limit)

            render(conn, "query_for_run.json",
              run_id: run_id,
              limit: limit,
              start: start,
              queries: queries,
              prev_start: prev_start,
              prev_limit: prev_limit,
              next_start: next_start,
              next_limit: next_limit
            )
        end

      :error ->
        send_resp(conn, :bad_request, "The limit must be 1 ~ 30!")
    end
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

  @doc """
  Retrieve the portion of the list of queries from `start` - 1 up to a number of
  `limit` queries.
  Calculate the start and limit of the previous/next page so that none of the
  entries would overlap with the current page.
  """
  def paginate(queries, start, limit) do
    len = length(queries)
    finish = min(start + limit - 1, len)
    retrieved_queries = Enum.slice(queries, start - 1, max(finish - start + 1, 0))
    prev_start = max(1, start - limit)
    prev_limit = start - prev_start
    next_start = finish + 1
    next_limit = min(limit, len - finish)
    {retrieved_queries, prev_start, prev_limit, next_start, next_limit}
  end

  defp limit_start(params) do
    case String.to_integer(params["limit"] || "30") do
      limit when limit in 1..30 ->
        start = String.to_integer(params["start"] || "1")
        {:ok, {limit, start}}

      _ ->
        :error
    end
  end
end
