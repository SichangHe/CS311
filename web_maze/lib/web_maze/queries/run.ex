defmodule WebMaze.Queries.Run do
  use Ecto.Schema
  import Ecto.Changeset
  alias WebMaze.Queries.Query

  schema "runs" do
    field :name, :string
    has_many :query, Query

    timestamps()
  end

  @doc false
  def changeset(run, attrs) do
    run
    |> cast(attrs, [:name])
    |> validate_required([:name])
  end
end
