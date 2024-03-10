# replace-files-with-itself

## About

Copies a list of files provided via parameter, and replaces the original with the new copy.

## Building

```
$env:RUSTFLAGS="-C panic=abort -C lto=fat -C embed-bitcode=yes"; cargo +nightly bloat -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target i686-pc-windows-gnu --profile profile
```