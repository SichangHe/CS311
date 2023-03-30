defmodule NetMaze.GenServer.State do
  @moduledoc """
  State of a `NetMaze.GenServer`.
  - `ip`: IP address to connect to.
  - `message`: message to send.
  - `primary`: primary connection socket.
  - `secondary`: map of secondary connections: port number => socket.
  """
  defstruct [:ip, :message, :primary, :secondary, :inform]

  @type t :: %__MODULE__{
          ip: charlist,
          message: String.t(),
          primary: port,
          secondary: %{non_neg_integer => port},
          inform: [pid]
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
  @spec init(args) :: {:ok, nil}
  def init(args) do
    send(self(), {:init, args})
    {:ok, nil}
  end

  @impl true
  @spec handle_info(any, State.t()) :: {:noreply, State.t()} | {:stop, :normal, State.t()}
  def handle_info({:init, args}, _state) do
    ip = Keyword.get(args, :ip) |> String.to_charlist()
    port = Keyword.get(args, :port)
    message = Keyword.get(args, :message) |> encode
    socket = connect_send(ip, port, message)
    {:noreply, %State{ip: ip, message: message, primary: socket, secondary: %{}, inform: []}}
  end

  def handle_info({:tcp, socket, message}, state) do
    message = String.trim(message)

    from_primary = socket == state.primary

    if from_primary do
      Logger.info("Received `#{message}` from primary connection.")
    else
      Logger.info("Received `#{message}`.")
    end

    decode(message, state)
  end

  def handle_info({:tcp_closed, socket}, state) do
    if socket == state.primary do
      Logger.info("Primary connection closed.")
      stop(state)
    else
      Logger.info("A secondary connection closed.")
      {:noreply, state}
    end
  end

  def handle_info(info, state) do
    dbg(info)
    {:noreply, state}
  end

  @impl true
  def handle_call(:wait, from, state) do
    {:noreply, update_in(state.inform, fn list -> [from | list] end)}
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
    GenServer.start_link(__MODULE__, args, name: __MODULE__)
  end

  @spec connect_send(charlist, non_neg_integer, String.t()) :: port
  defp connect_send(ip, port, message) do
    {:ok, socket} = :gen_tcp.connect(ip, port, [:binary, packet: :line])
    :ok = :gen_tcp.send(socket, message)
    Logger.info("Sent `#{message}` to #{ip}:#{port}.")
    socket
  end

  @spec encode(String.t()) :: String.t()
  defp encode(message) do
    "id " <> message <> "\n"
  end

  defp decode("query " <> port_str, state) do
    port = String.to_integer(port_str)

    case state.secondary[port] do
      nil ->
        # No existing connection to this new port.
        # Establish a new connection.
        socket = connect_send(state.ip, port, state.message)
        {:noreply, update_in(state.secondary, &Map.put(&1, port, socket))}

      existing_socket ->
        # Connection to this port exists.
        # Resend the identification.
        :gen_tcp.send(existing_socket, state.message)
        Logger.info("Resent `#{state.message}` to #{state.ip}:#{port}.")
        {:noreply, state}
    end
  end

  defp decode("status " <> status, state) do
    Logger.warn("NetMaze.GenServer stopping at `#{status}`.")
    stop(state)
  end

  defp decode("listen " <> port_str, state) do
    port = String.to_integer(port_str)
    {:ok, listen_socket} = :gen_tcp.listen(port, [:binary, packet: :line, reuseaddr: true])
    Logger.info("Listening at port #{port_str} as requested.")
    {:ok, socket} = :gen_tcp.accept(listen_socket)
    Logger.info("Port #{port_str} received connection.")
    {:noreply, update_in(state.secondary, &Map.put(&1, port, socket))}
  end

  @spec stop(State.t()) :: {:stop, :normal, State.t()}
  defp stop(state) do
    for pid <- state.inform do
      GenServer.reply(pid, :stop)
    end

    {:stop, :normal, state}
  end
end
