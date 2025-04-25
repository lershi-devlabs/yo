class YoBin < Formula
  version '1.0.10'
  desc "Ask your terminal anything using AI."
  homepage "https://github.com/montekkundan/yo"

  if OS.mac?
    url "https://github.com/Montekkundan/yo/releases/download/1.0.10/yo-1.0.10-x86_64-apple-darwin.tar.gz"
    sha256 "0000000000000000000000000000000000000000000000000000000000000000"
  elsif OS.linux?
    url "https://github.com/Montekkundan/yo/releases/download/1.0.10/yo-1.0.10-x86_64-unknown-linux-musl.tar.gz"
    sha256 "0000000000000000000000000000000000000000000000000000000000000000"
  end

  def install
    bin.install "yo"
  end
end
