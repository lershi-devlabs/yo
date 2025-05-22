class YoBin < Formula
  version '1.3.0'
  desc "Ask your terminal anything using AI."
  homepage "https://github.com/montekkundan/yo"

  if OS.mac?
    url "https://github.com/lershi-devlabs/yo/releases/download/1.3.0/yo-1.3.0-x86_64-apple-darwin.tar.gz"
    sha256 "ba667ad91530b150047424cb695442ad9571c1701d221d78a9d51c1245a81e02"
  elsif OS.linux?
    url "https://github.com/lershi-devlabs/yo/releases/download/1.3.0/yo-1.3.0-x86_64-unknown-linux-musl.tar.gz"
    sha256 "ba667ad91530b150047424cb695442ad9571c1701d221d78a9d51c1245a81e02"
  end

  def install
    bin.install "yo"
  end
end
