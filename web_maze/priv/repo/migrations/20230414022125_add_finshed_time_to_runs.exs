defmodule WebMaze.Repo.Migrations.AddFinshedTimeToRuns do
  use Ecto.Migration

  def change do
    alter table(:runs) do
      add :finished, :utc_datetime
    end
  end
end
