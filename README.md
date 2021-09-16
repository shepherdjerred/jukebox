# jukebox

## Development Requirements
### macos
```bash
brew install pcsc-lite openssl@1.1 pkg-config

export PKG_CONFIG=/opt/homebrew/bin/pkg-config
set -gx PKG_CONFIG_PATH "/opt/homebrew/opt/pcsc-lite/lib/pkgconfig:/opt/homebrew/opt/openssl@1.1/lib/pkgconfig"
```

