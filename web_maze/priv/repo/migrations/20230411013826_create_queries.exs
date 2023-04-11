defmodule WebMaze.Repo.Migrations.CreateQueries do
  use Ecto.Migration

  def change do
    create table(:queries) do
      add :connection_source, :string
      add :connection_port, :integer
      add :query_target, :integer

      timestamps()
    end
  end
end
