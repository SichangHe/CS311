defmodule NetMazeTest do
  use ExUnit.Case
  doctest NetMaze

  test "true" do
    assert true === true
  end

  test "sleep" do
    :timer.sleep(5000)
  end
end
