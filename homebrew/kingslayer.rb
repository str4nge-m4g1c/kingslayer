class Kingslayer < Formula
  desc "Terminal UI implementation of the cooperative card game Regicide"
  homepage "https://github.com/str4nge-m4g1c/regicide-tui"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/str4nge-m4g1c/regicide-tui/releases/download/v#{version}/kingslayer-macos-aarch64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_ARM64_MACOS" # Run: shasum -a 256 kingslayer-macos-aarch64.tar.gz
    else
      url "https://github.com/str4nge-m4g1c/regicide-tui/releases/download/v#{version}/kingslayer-macos-x86_64.tar.gz"
      sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_X86_64_MACOS" # Run: shasum -a 256 kingslayer-macos-x86_64.tar.gz
    end
  end

  on_linux do
    url "https://github.com/str4nge-m4g1c/regicide-tui/releases/download/v#{version}/kingslayer-linux-x86_64.tar.gz"
    sha256 "REPLACE_WITH_ACTUAL_SHA256_FOR_LINUX" # Run: shasum -a 256 kingslayer-linux-x86_64.tar.gz
  end

  def install
    bin.install "kingslayer"
  end

  test do
    # Test that the binary exists and is executable
    assert_predicate bin/"kingslayer", :exist?
    assert_predicate bin/"kingslayer", :executable?
  end
end
