defmodule WebMazeWeb.PageControllerTest do
  use WebMazeWeb.ConnCase

  test "GET /", %{conn: conn} do
    conn = get(conn, "/")
    assert html_response(conn, 200) =~ "Welcome to Phoenix!"
  end

  test "POST /api/run/someone", %{conn: conn} do
    conn = post(conn, "/api/run/someone")
    assert json_response(conn, 200) == %{"runId" => "1"}
  end
end
