defmodule WebMaze.QueriesFixtures do
  @moduledoc """
  This module defines test helpers for creating
  entities via the `WebMaze.Queries` context.
  """

  @doc """
  Generate a query.
  """
  def query_fixture(attrs \\ %{}) do
    {:ok, query} =
      attrs
      |> Enum.into(%{
        connection_port: 42,
        connection_source: "some connection_source",
        query_target: 42
      })
      |> WebMaze.Queries.create_query()

    query
  end
end
