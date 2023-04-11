defmodule WebMazeWeb.QueryControllerTest do
  use WebMazeWeb.ConnCase

  import WebMaze.QueriesFixtures

  alias WebMaze.Queries.Query

  @create_attrs %{
    connection_port: 42,
    connection_source: "some connection_source",
    query_target: 42
  }
  @update_attrs %{
    connection_port: 43,
    connection_source: "some updated connection_source",
    query_target: 43
  }
  @invalid_attrs %{connection_port: nil, connection_source: nil, query_target: nil}

  setup %{conn: conn} do
    {:ok, conn: put_req_header(conn, "accept", "application/json")}
  end

  describe "index" do
    test "lists all queries", %{conn: conn} do
      conn = get(conn, Routes.query_path(conn, :index))
      assert json_response(conn, 200)["data"] == []
    end
  end

  describe "create query" do
    test "renders query when data is valid", %{conn: conn} do
      conn = post(conn, Routes.query_path(conn, :create), query: @create_attrs)
      assert %{"id" => id} = json_response(conn, 201)["data"]

      conn = get(conn, Routes.query_path(conn, :show, id))

      assert %{
               "id" => ^id,
               "connection_port" => 42,
               "connection_source" => "some connection_source",
               "query_target" => 42
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn} do
      conn = post(conn, Routes.query_path(conn, :create), query: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "update query" do
    setup [:create_query]

    test "renders query when data is valid", %{conn: conn, query: %Query{id: id} = query} do
      conn = put(conn, Routes.query_path(conn, :update, query), query: @update_attrs)
      assert %{"id" => ^id} = json_response(conn, 200)["data"]

      conn = get(conn, Routes.query_path(conn, :show, id))

      assert %{
               "id" => ^id,
               "connection_port" => 43,
               "connection_source" => "some updated connection_source",
               "query_target" => 43
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn, query: query} do
      conn = put(conn, Routes.query_path(conn, :update, query), query: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "delete query" do
    setup [:create_query]

    test "deletes chosen query", %{conn: conn, query: query} do
      conn = delete(conn, Routes.query_path(conn, :delete, query))
      assert response(conn, 204)

      assert_error_sent 404, fn ->
        get(conn, Routes.query_path(conn, :show, query))
      end
    end
  end

  defp create_query(_) do
    query = query_fixture()
    %{query: query}
  end
end
