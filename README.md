# jukebox

## Development Requirements
This program is meant to be built on macOS arm64 targeting linux x84-64. The cross compilation requires several packages to be installed.

### macos
```bash
rustup target add armv7-unknown-linux-musleabihf

brew tap osx-cross/arm
brew tap MarioSchwalbe/gcc-musl-cross
brew install arm-gcc-bin llvm arm-linux-gnueabihf-binutils
brew install pcsc-lite openssl@1.1 pkg-config
brew install gcc-8-musl-cross

set -gx LDFLAGS "-L/opt/homebrew/opt/pcsc-lite/lib"
set -gx CPPFLAGS "-I/opt/homebrew/opt/pcsc-lite/include"

export PKG_CONFIG=/opt/homebrew/bin/pkg-config
set -gx PKG_CONFIG_PATH "/opt/homebrew/opt/pcsc-lite/lib/pkgconfig:/opt/homebrew/opt/openssl@1.1/lib/pkgconfig"
```

