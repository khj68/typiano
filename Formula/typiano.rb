class Typiano < Formula
  desc "Turn your typing into a piano performance"
  homepage "https://github.com/khj68/typiano"
  url "https://github.com/khj68/typiano/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "PLACEHOLDER"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std.cargo_args, "--path", "."
  end

  def caveats
    <<~EOS
      typiano needs Accessibility permission to detect keystrokes.

      macOS: System Settings → Privacy & Security → Accessibility
             → Enable your terminal app

      Usage:
        typiano on      # Start
        typiano off     # Stop
        typiano list    # Show songs
    EOS
  end

  test do
    assert_match "typiano", shell_output("#{bin}/typiano --help")
  end
end
