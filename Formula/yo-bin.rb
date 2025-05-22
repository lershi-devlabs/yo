class YoBin < Formula
  version '1.3.4'
  desc "Ask your terminal anything using AI."
  homepage "https://github.com/montekkundan/yo"

  if OS.mac?
    url "https://github.com/lershi-devlabs/yo/releases/download/1.3.4/yo-1.3.4-x86_64-apple-darwin.tar.gz"
    sha256 "37bfc7a099651dbc04950b5564daebda8e4687611f1f3cf6044c67fda8124522"
  elsif OS.linux?
    url "https://github.com/lershi-devlabs/yo/releases/download/1.3.4/yo-1.3.4-x86_64-unknown-linux-musl.tar.gz"
    sha256 "37bfc7a099651dbc04950b5564daebda8e4687611f1f3cf6044c67fda8124522"
  end

  def install
    bin.install "yo"
  end
end
