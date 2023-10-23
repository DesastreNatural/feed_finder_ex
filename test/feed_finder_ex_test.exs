defmodule FeedFinderExTest do
  use ExUnit.Case
  doctest FeedFinderEx

  test "basic test" do
    url = "https://example.com/example"
    html_fake_data = """
      <html>
          <head>
              <title>Example</title>
              <link rel="alternate" href="/posts.rss" type="application/rss+xml" />
          </head>
          <body>
              My fun page with a feed.
          </body>
      </html>
    """
    test_data = FeedFinderEx.find_feeds(url,html_fake_data)
    assert  test_data == {:ok,[%{"feed_type" => "Rss","title" => "", "url" => "https://example.com/posts.rss"}]}
  end

  test "test with NYT" do
    url = "https://www.nytimes.com/"
    {:ok, response} = HTTPoison.get(url)
    test_data = FeedFinderEx.find_feeds(url,response.body)
    assert  test_data == {:ok,[%{"feed_type" => "Rss","title" => "RSS", "url" => "https://rss.nytimes.com/services/xml/rss/nyt/HomePage.xml"}]}
  end

end
