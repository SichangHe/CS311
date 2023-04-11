defmodule WebMaze.Repo.Migrations.CreateRuns do
  use Ecto.Migration

  def change do
    create table(:runs) do
      add :name, :string

      timestamps()
    end

    alter table(:queries) do
      add :run_id, references(:runs, on_delete: :delete_all)
    end
  end
end
