defmodule WebMazeWeb.QueryController do
  use WebMazeWeb, :controller
  use PhoenixSwagger
  require Logger

  alias WebMaze.Queries
  alias WebMaze.Queries.Query

  action_fallback WebMazeWeb.FallbackController

  swagger_path :finished do
    get("/api/list")
    summary("List finished runs")
    description("Returns a list of finished runs with pagination")

    parameters do
      start(:query, :integer, "The start index for pagination", required: false)
      limit(:query, :integer, "The number of items per page", required: false)
    end

    response(200, "OK", Schema.ref(:FinishedResponse))
    response(400, "Bad Request: The limit must be 1 ~ 30!")
  end

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

  swagger_path :run do
    post("/api/run/{id}")
    summary("Run a query")
    description("Runs a query with the given ID")

    parameters do
      id(:path, :string, "The ID of the query to run", required: true)
    end

    response(200, "OK", Schema.ref(:RunResponse))
  end

  def run(conn, %{"id" => id}) do
    {:ok, run} = Queries.create_run(%{name: id})

    query_call = fn connection_source, connection_port, query_target ->
      case Queries.create_query(%{
             connection_source: connection_source,
             connection_port: connection_port,
             query_target: query_target,
             run_id: run.id
           }) do
        {:error, changeset} ->
          Logger.error("Error creating query with #{inspect(changeset)}")

        {:ok, _} ->
          :ok
      end
    end

    finish_call = fn -> Queries.finish_run(run) end

    args = [
      ip: System.get_env("IP") || "67.159.95.167",
      port: System.get_env("PORT") || 51300,
      message: id,
      query_call: query_call,
      finish_call: finish_call
    ]

    DynamicSupervisor.start_child(NetMaze.Supervisor, {NetMaze.GenServer, args})

    render(conn, "run.json", run: run)
  end

  swagger_path :index do
    get("/api/queries")
    summary("List queries for a run")
    description("Returns a list of queries for a given run with pagination")

    parameters do
      run(:query, :string, "The run ID list queries for", required: true)
      start(:query, :integer, "The start index for pagination", required: false)
      limit(:query, :integer, "The number of items per page", required: false)
    end

    response(200, "OK", Schema.ref(:IndexResponse))
    response(204, "No Content: Run not finished")
    response(400, "Bad Request: The limit must be 1 ~ 30!")
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

  def swagger_definitions do
    %{
      FinishedResponse:
        swagger_schema do
          title("List of runs")
          description("A paginated list of finished runs")

          properties do
            limit(:integer, "The number of items per page")
            start(:integer, "The start index for pagination")
            runIds(array(:string), "An array of run IDs")
            prev(:string, "The URL for the previous page")
            next(:string, "The URL for the next page")
          end
        end,
      RunResponse:
        swagger_schema do
          title("Run response")
          description("Information about the run that was created")

          properties do
            runId(:string, "The ID of the run")
          end
        end,
      IndexResponse:
        swagger_schema do
          title("Index Response")
          description("A paginated list of queries for a given run")

          properties do
            runId(:string, "The ID of the run")
            limit(:integer, "The number of items per page")
            start(:integer, "The start index for pagination")
            queries(array(:Query), "The list of queries that belong to this run")
            prev(:string, "The URL for the previous page")
            next(:string, "The URL for the next page")
          end
        end,
      Query:
        swagger_schema do
          properties do
            id(:integer, "The ID of the query")
            connection_source(:string, "The connection source of the query")
            connection_port(:integer, "The connection port of the query")
            query_target(:integer, "The query target of the query")
            run_id(:integer, "The ID of the run that this query belongs to")
          end
        end
    }
  end
end
