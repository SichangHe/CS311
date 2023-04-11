defmodule WebMaze.Queries.Query do
  use Ecto.Schema
  import Ecto.Changeset
  alias WebMaze.Queries.Run

  schema "queries" do
    field :connection_port, :integer
    field :connection_source, :string
    field :query_target, :integer
    belongs_to :run, Run

    timestamps()
  end

  @doc false
  def changeset(query, attrs) do
    query
    |> cast(attrs, [:connection_source, :connection_port, :query_target, :run_id])
    |> validate_required([:connection_source, :connection_port, :query_target, :run_id])
  end
end
