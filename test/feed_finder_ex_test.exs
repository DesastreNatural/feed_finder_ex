defmodule FeedFinderExTest do
  use ExUnit.Case
  doctest FeedFinderEx

  test "greets the world" do
    url = "https://www.nytimes.com/"
    {:ok, response} = HTTPoison.get(url)
    test_data = FeedFinderEx.find_feeds(url,response.body)
    IO.inspect test_data
    assert  test_data == :world
  end
end
