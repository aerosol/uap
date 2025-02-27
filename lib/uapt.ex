defmodule UapNif do
  use Rustler, otp_app: :uapt, crate: "uap"

  # Fallback if the NIF fails to load
  def parse_ua(_user_agent), do: :erlang.nif_error(:nif_not_loaded)
end
