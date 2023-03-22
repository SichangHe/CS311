defmodule NetMazeTest do
  use ExUnit.Case
  doctest NetMaze

  test "greets the world" do
    assert NetMaze.hello() == :world
  end
end
