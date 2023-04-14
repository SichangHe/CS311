defmodule WebMaze.Queries.Run do
  use Ecto.Schema
  import Ecto.Changeset
  alias WebMaze.Queries.Query

  schema "runs" do
    field :name, :string
    field :finished, :utc_datetime
    has_many :query, Query

    timestamps()
  end

  @doc false
  def changeset(run, attrs) do
    run
    |> cast(attrs, [:name, :finished])
    |> validate_required([:name])
  end
end
