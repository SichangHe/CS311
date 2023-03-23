defmodule NetMaze.GenServer do
  use GenServer, restart: :transient

  @impl true
  def init(args) do
    ip = Keyword.get(args, :ip) |> String.to_charlist()
    port = Keyword.get(args, :port)
    message = Keyword.get(args, :message)
    {:ok, socket} = :gen_tcp.connect(ip, port, [:binary, packet: :line])
    :ok = :gen_tcp.send(socket, message <> "\n")
    IO.puts("Sent message")
    {:ok, socket}
  end

  @impl true
  def handle_info({:tcp, socket, message}, state) do
    IO.puts("Received #{String.trim(message)}.")
    ^state = socket
    {:noreply, state}
  end

  def handle_info({:tcp_closed, socket}, state) do
    IO.puts("Socket closed.")
    ^state = socket
    {:stop, :normal, state}
  end

  def handle_info(info, state) do
    dbg(info)
    {:noreply, state}
  end

  def start_link(opts) do
    GenServer.start_link(__MODULE__, opts, [])
  end
end
