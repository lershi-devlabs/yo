class YoBin < Formula
  version '1.2.0'
  desc "Ask your terminal anything using AI."
  homepage "https://github.com/montekkundan/yo"

  if OS.mac?
    url "https://github.com/lershi-devlabs/yo/releases/download/1.2.0/yo-1.2.0-x86_64-apple-darwin.tar.gz"
    sha256 "030306cbd39195311d3bb196c8218e5e028f79304355967eec2d7a5cc4efe8f0"
  elsif OS.linux?
    url "https://github.com/lershi-devlabs/yo/releases/download/1.2.0/yo-1.2.0-x86_64-unknown-linux-musl.tar.gz"
    sha256 "030306cbd39195311d3bb196c8218e5e028f79304355967eec2d7a5cc4efe8f0"
  end

  def install
    bin.install "yo"
  end
end
