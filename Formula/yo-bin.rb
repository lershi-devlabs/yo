class YoBin < Formula
  version '1.2.0'
  desc "Ask your terminal anything using AI."
  homepage "https://github.com/montekkundan/yo"

  if OS.mac?
    url "https://github.com/lershi-devlabs/yo/releases/download/1.2.0/yo-1.2.0-x86_64-apple-darwin.tar.gz"
    sha256 "a38968482cd81019ff12d479e480e353b54a017a6b74af649486da7ebb42f199"
  elsif OS.linux?
    url "https://github.com/lershi-devlabs/yo/releases/download/1.2.0/yo-1.2.0-x86_64-unknown-linux-musl.tar.gz"
    sha256 "a38968482cd81019ff12d479e480e353b54a017a6b74af649486da7ebb42f199"
  end

  def install
    bin.install "yo"
  end
end
