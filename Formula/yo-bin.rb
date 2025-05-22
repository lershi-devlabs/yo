class YoBin < Formula
  version '1.3.2'
  desc "Ask your terminal anything using AI."
  homepage "https://github.com/montekkundan/yo"

  if OS.mac?
    url "https://github.com/lershi-devlabs/yo/releases/download/1.3.2/yo-1.3.2-x86_64-apple-darwin.tar.gz"
    sha256 "57cec65613fd202307bf7390b884f9f65c8258d2e9cc52b86c768a3211e5ba7a"
  elsif OS.linux?
    url "https://github.com/lershi-devlabs/yo/releases/download/1.3.2/yo-1.3.2-x86_64-unknown-linux-musl.tar.gz"
    sha256 "57cec65613fd202307bf7390b884f9f65c8258d2e9cc52b86c768a3211e5ba7a"
  end

  def install
    bin.install "yo"
  end
end
