class YoBin < Formula
  version '1.0.0'
  desc "Ask your terminal anything using AI."
  homepage "https://github.com/montekkundan/yo"

  if OS.mac?
    url "https://github.com/montekkundan/yo/releases/download/#{version}/yo-#{version}-x86_64-apple-darwin.tar.gz"
    sha256 "REPLACE_WITH_MAC_SHA256"
  elsif OS.linux?
    url "https://github.com/montekkundan/yo/releases/download/#{version}/yo-#{version}-x86_64-unknown-linux-musl.tar.gz"
    sha256 "REPLACE_WITH_LINUX_SHA256"
  end

  def install
    bin.install "yo"
  end
end
