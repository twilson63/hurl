class Hurl < Formula
  desc "Modern HTTP CLI - blazingly fast, user-friendly HTTP client in Rust"
  homepage "https://github.com/hurl/hurl"
  url "https://github.com/hurl/hurl/releases/download/v0.1.0/hurl-0.1.0-x86_64-apple-darwin.tar.gz"
  sha256 "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
  license "MIT", "Apache-2.0"

  depends_on "rust" => :build

  def install
    bin.install "hurl"
    
    bash_completion.install "completions/hurl.bash" => "hurl"
    zsh_completion.install "completions/hurl.zsh" => "_hurl"
    fish_completion.install "completions/hurl.fish" => "hurl.fish"
    
    man1.install "man/hurl.1"
  end

  test do
    system "#{bin}/hurl", "--version"
  end
end
