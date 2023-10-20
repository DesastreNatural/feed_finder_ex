defmodule FeedFinderEx do
  use Rustler, otp_app: :feed_finder_ex

  def find_feeds(_arg1,_arg2), do: :erlang.nif_error(:nif_not_loaded)
end
