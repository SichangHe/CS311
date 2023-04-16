defmodule WebMaze.Client do
  alias WebMaze.Queries

  def mean_variance() do
    runs = Queries.list_runs()

    queries =
      runs
      |> Enum.filter(fn run -> run.finished end)
      |> Enum.map(fn run -> Queries.queries_for_run(run) end)

    lens = queries |> Enum.map(fn q -> length(q) end)
    mean = Enum.sum(lens) / length(lens)

    variance =
      (lens |> Enum.map(fn len -> :math.pow(len - mean, 2) end) |> Enum.sum()) / length(lens)

    {mean, variance}
  end
end
