class YoBin < Formula
  version '1.2.0'
  desc "Ask your terminal anything using AI."
  homepage "https://github.com/montekkundan/yo"

  if OS.mac?
    url "https://github.com/Montekkundan/yo/releases/download/1.1.1/yo-1.1.1-x86_64-apple-darwin.tar.gz"
    sha256 "9551d1f7dea119861d6f931918014afa66b6fcddacad032c871bddcadc282059"
  elsif OS.linux?
    url "https://github.com/Montekkundan/yo/releases/download/1.1.1/yo-1.1.1-x86_64-unknown-linux-musl.tar.gz"
    sha256 "9551d1f7dea119861d6f931918014afa66b6fcddacad032c871bddcadc282059"
  end

  def install
    bin.install "yo"
  end
end
