defmodule NetMaze.GenServer.State do
  @moduledoc """
  State of a `NetMaze.GenServer`.
  - `ip`: IP address to connect to.
  - `message`: message to send.
  - `primary`: primary connection socket.
  - `secondary`: map of secondary connections: port number => socket.
  """
  defstruct [:ip, :message, :primary, :secondary]

  @type t :: %__MODULE__{
          ip: charlist,
          message: String.t(),
          primary: port,
          secondary: %{non_neg_integer => port}
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
    {:ok, socket} = :gen_tcp.connect(ip, port, [:binary, packet: :line])
    send(self(), :init)
    {:ok, %State{ip: ip, message: message, primary: socket, secondary: %{}}}
  end

  @impl true
  @spec handle_info(any, State.t() | nil) :: {:noreply, State.t()} | {:stop, :normal, State.t()}
  def handle_info(:init, state) do
    case :gen_tcp.send(state.primary, state.message) do
      {:error, error} ->
        try do
          Logger.error("#{inspect(error)} init.")
        catch
          kind, _value when kind in [:exit, :throw] ->
            Logger.error("Error init.")
        end

        {:stop, :normal, nil}

      :ok ->
        {:noreply, state}
    end
  end

  def handle_info({:tcp, _socket, message}, state) do
    message = String.trim(message)

    case message do
      "query " <> port_str ->
        port = String.to_integer(port_str)

        case state.secondary[port] do
          nil ->
            # No existing connection to this new port.
            # Establish a new connection.
            case connect_send(state.ip, port, state.message) do
              {:error, error} ->
                try do
                  Logger.error("#{inspect(error)} trying to connect to #{state.ip}:#{port}.")
                catch
                  kind, _value when kind in [:exit, :throw] ->
                    Logger.error("Error trying to connect to #{state.ip}:#{port}.")
                end

                {:stop, :normal, state}

              {:ok, socket} ->
                {:noreply, update_in(state.secondary, &Map.put(&1, port, socket))}
            end

          existing_socket ->
            # Connection to this port exists.
            # Resend the identification.
            :gen_tcp.send(existing_socket, state.message)
            {:noreply, state}
        end

      "status " <> status ->
        Logger.warn("NetMaze.GenServer stopping at `#{status}`.")
        {:stop, :normal, state}
    end
  end

  def handle_info({:tcp_closed, socket}, state) do
    if socket == state.primary do
      {:stop, :normal, state}
    else
      {:noreply, state}
    end
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
    GenServer.start_link(__MODULE__, args)
  end

  @spec connect_send(charlist, non_neg_integer, String.t()) :: {:ok, port} | {:error, any}
  defp connect_send(ip, port, message) do
    with {:ok, socket} <- :gen_tcp.connect(ip, port, [:binary, packet: :line]),
         :ok <- :gen_tcp.send(socket, message) do
      {:ok, socket}
    else
      error -> {:error, error}
    end
  end

  @spec encode(String.t()) :: String.t()
  defp encode(message) do
    "id " <> message <> "\n"
  end
end
