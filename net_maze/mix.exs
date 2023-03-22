defmodule NetMaze.MixProject do
  use Mix.Project

  def project do
    [
      app: :net_maze,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger],
      mod:
        {NetMaze,
         [
           ip: System.get_env("IP") || "127.0.0.1",
           port: System.get_env("PORT") || 58200,
           message: System.get_env("MESSAGE") || "youknowwho"
         ]}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
    ]
  end
end
