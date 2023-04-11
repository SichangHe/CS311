defmodule WebMazeWeb.PageController do
  use WebMazeWeb, :controller

  def index(conn, _params) do
    render(conn, "index.html")
  end
end
