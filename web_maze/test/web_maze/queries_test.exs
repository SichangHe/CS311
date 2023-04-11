defmodule WebMaze.QueriesTest do
  use WebMaze.DataCase

  alias WebMaze.Queries

  describe "queries" do
    alias WebMaze.Queries.Query

    import WebMaze.QueriesFixtures

    @invalid_attrs %{connection_port: nil, connection_source: nil, query_target: nil}

    test "list_queries/0 returns all queries" do
      query = query_fixture()
      assert Queries.list_queries() == [query]
    end

    test "get_query!/1 returns the query with given id" do
      query = query_fixture()
      assert Queries.get_query!(query.id) == query
    end

    test "create_query/1 with valid data creates a query" do
      valid_attrs = %{connection_port: 42, connection_source: "some connection_source", query_target: 42}

      assert {:ok, %Query{} = query} = Queries.create_query(valid_attrs)
      assert query.connection_port == 42
      assert query.connection_source == "some connection_source"
      assert query.query_target == 42
    end

    test "create_query/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Queries.create_query(@invalid_attrs)
    end

    test "update_query/2 with valid data updates the query" do
      query = query_fixture()
      update_attrs = %{connection_port: 43, connection_source: "some updated connection_source", query_target: 43}

      assert {:ok, %Query{} = query} = Queries.update_query(query, update_attrs)
      assert query.connection_port == 43
      assert query.connection_source == "some updated connection_source"
      assert query.query_target == 43
    end

    test "update_query/2 with invalid data returns error changeset" do
      query = query_fixture()
      assert {:error, %Ecto.Changeset{}} = Queries.update_query(query, @invalid_attrs)
      assert query == Queries.get_query!(query.id)
    end

    test "delete_query/1 deletes the query" do
      query = query_fixture()
      assert {:ok, %Query{}} = Queries.delete_query(query)
      assert_raise Ecto.NoResultsError, fn -> Queries.get_query!(query.id) end
    end

    test "change_query/1 returns a query changeset" do
      query = query_fixture()
      assert %Ecto.Changeset{} = Queries.change_query(query)
    end
  end

  describe "runs" do
    alias WebMaze.Queries.Run

    import WebMaze.QueriesFixtures

    @invalid_attrs %{name: nil}

    test "list_runs/0 returns all runs" do
      run = run_fixture()
      assert Queries.list_runs() == [run]
    end

    test "get_run!/1 returns the run with given id" do
      run = run_fixture()
      assert Queries.get_run!(run.id) == run
    end

    test "create_run/1 with valid data creates a run" do
      valid_attrs = %{name: "some name"}

      assert {:ok, %Run{} = run} = Queries.create_run(valid_attrs)
      assert run.name == "some name"
    end

    test "create_run/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Queries.create_run(@invalid_attrs)
    end

    test "update_run/2 with valid data updates the run" do
      run = run_fixture()
      update_attrs = %{name: "some updated name"}

      assert {:ok, %Run{} = run} = Queries.update_run(run, update_attrs)
      assert run.name == "some updated name"
    end

    test "update_run/2 with invalid data returns error changeset" do
      run = run_fixture()
      assert {:error, %Ecto.Changeset{}} = Queries.update_run(run, @invalid_attrs)
      assert run == Queries.get_run!(run.id)
    end

    test "delete_run/1 deletes the run" do
      run = run_fixture()
      assert {:ok, %Run{}} = Queries.delete_run(run)
      assert_raise Ecto.NoResultsError, fn -> Queries.get_run!(run.id) end
    end

    test "change_run/1 returns a run changeset" do
      run = run_fixture()
      assert %Ecto.Changeset{} = Queries.change_run(run)
    end
  end
end
