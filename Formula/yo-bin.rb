class YoBin < Formula
  version '1.0.11'
  desc "Ask your terminal anything using AI."
  homepage "https://github.com/montekkundan/yo"

  if OS.mac?
    url "https://github.com/Montekkundan/yo/releases/download/1.0.11/yo-1.0.11-x86_64-apple-darwin.tar.gz"
    sha256 "1672425bdafb5660b1baf88c22f8e5925d628485381700b0d98ba9c060f9e2df"
  elsif OS.linux?
    url "https://github.com/Montekkundan/yo/releases/download/1.0.11/yo-1.0.11-x86_64-unknown-linux-musl.tar.gz"
    sha256 "1672425bdafb5660b1baf88c22f8e5925d628485381700b0d98ba9c060f9e2df"
  end

  def install
    bin.install "yo"
  end
end
