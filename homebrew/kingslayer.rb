class Kingslayer < Formula
  desc "Terminal UI implementation of the cooperative card game Regicide"
  homepage "https://github.com/str4nge-m4g1c/kingslayer"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/str4nge-m4g1c/kingslayer/releases/download/v#{version}/kingslayer-macos-aarch64.tar.gz"
      sha256 "e71a0b6ee9f588f482c03448d31a409d2ea26212c0e08d5f89972299b0660e41"
    else
      url "https://github.com/str4nge-m4g1c/kingslayer/releases/download/v#{version}/kingslayer-macos-x86_64.tar.gz"
      sha256 "6414de3e3002c7f2f1a7157b87ebb91dda6484bf6586b060491e781b3a924ed5"
    end
  end

  on_linux do
    url "https://github.com/str4nge-m4g1c/kingslayer/releases/download/v#{version}/kingslayer-linux-x86_64.tar.gz"
    sha256 "e15aa0a18d44fd28aa68a6081cb4452f4662ca65922b3b7bd8305c53ad169144"
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
