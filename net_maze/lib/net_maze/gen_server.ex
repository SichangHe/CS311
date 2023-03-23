defmodule NetMaze.GenServer.State do
  defstruct [:ip, :port, :message, :primary, :secondary]

  @type t :: %__MODULE__{
          ip: charlist,
          port: non_neg_integer,
          message: String.t(),
          primary: port,
          secondary: port | nil
        }
end

defmodule NetMaze.GenServer do
  @moduledoc """
  A generic server that establishes a connection to the given IP and port,
  and sends the given message as soon as the connection is established.
  """
  use GenServer, restart: :transient
  require Logger
  alias __MODULE__.State

  @type args :: [ip: String.t(), port: non_neg_integer, message: String.t()]

  @impl true
  @spec init(args) :: {:ok, State.t() | {:inet, atom, any}}
  def init(args) do
    ip = Keyword.get(args, :ip) |> String.to_charlist()
    port = Keyword.get(args, :port)
    message = Keyword.get(args, :message) |> encode
    Logger.info("Sent message")
    socket = connect_send(ip, port, message)
    {:ok, %State{ip: ip, port: port, message: message, primary: socket, secondary: nil}}
  end

  @impl true
  def handle_info({:tcp, socket, message}, state) do
    message = String.trim(message)

    if socket == state.primary do
      Logger.info("Received `#{message}` from primary connection.")
    else
      Logger.info("Received `#{message}`.")
    end

    case message do
      "query " <> port_str ->
        port = String.to_integer(port_str)
        socket = connect_send(state.ip, port, message)
        {:noreply, %State{state | secondary: socket}}

      "status " <> _status ->
        {:stop, :normal, state}
    end
  end

  def handle_info({:tcp_closed, socket}, state) do
    Logger.info("Socket closed.")
    ^socket = state.primary
    {:stop, :normal, state}
  end

  def handle_info(info, state) do
    dbg(info)
    {:noreply, state}
  end

  @doc """
  Start a `NetMaze.GenServer` instance.

  The `args` passed in should include:
  - `:ip`: the IP to connect to.
  - `:port`: the port to connect to.
  - `:message`: the message to send.
  """
  @spec start_link(args) :: {:ok, pid}
  def start_link(args) do
    GenServer.start_link(__MODULE__, args, [])
  end

  @spec connect_send(charlist, non_neg_integer, String.t()) :: port
  defp connect_send(ip, port, message) do
    {:ok, socket} = :gen_tcp.connect(ip, port, [:binary, packet: :line])
    :ok = :gen_tcp.send(socket, message)
    socket
  end

  @spec encode(String.t()) :: String.t()
  defp encode(message) do
    "id " <> message <> "\n"
  end
end
