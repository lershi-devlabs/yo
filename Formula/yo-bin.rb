class YoBin < Formula
  version '1.3.1'
  desc "Ask your terminal anything using AI."
  homepage "https://github.com/montekkundan/yo"

  if OS.mac?
    url "https://github.com/lershi-devlabs/yo/releases/download/1.3.1/yo-1.3.1-x86_64-apple-darwin.tar.gz"
    sha256 "63bfe9694531f01f4bea29bebd3e183413dfc13faf5456f9e27556097ce95cd8"
  elsif OS.linux?
    url "https://github.com/lershi-devlabs/yo/releases/download/1.3.1/yo-1.3.1-x86_64-unknown-linux-musl.tar.gz"
    sha256 "63bfe9694531f01f4bea29bebd3e183413dfc13faf5456f9e27556097ce95cd8"
  end

  def install
    bin.install "yo"
  end
end
