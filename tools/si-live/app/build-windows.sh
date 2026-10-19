#!/usr/bin/env bash
# Build si-companion.exe for Windows from WSL (native app spec §2) and put it,
# with the runtime DLL it needs, in tools/si-live/target/si-companion/.
#
#   tools/si-live/app/build-windows.sh            # release
#   tools/si-live/app/build-windows.sh --debug
#
# Needs the Rust target x86_64-pc-windows-gnullvm and llvm-mingw. Set
# LLVM_MINGW to its folder if it isn't the one below.
set -euo pipefail
cd "$(dirname "$0")/.."
LLVM_MINGW="${LLVM_MINGW:-$HOME/.local/toolchains/llvm-mingw-20261006-ucrt-ubuntu-22.04-x86_64}"
TARGET=x86_64-pc-windows-gnullvm
export CARGO_TARGET_X86_64_PC_WINDOWS_GNULLVM_LINKER="$LLVM_MINGW/bin/x86_64-w64-mingw32-clang"
profile=release
flags=(--release)
if [[ "${1:-}" == "--debug" ]]; then
  profile=debug
  flags=()
fi
cargo build -p si-app --target "$TARGET" "${flags[@]}"
out=target/si-companion
mkdir -p "$out"
cp "target/$TARGET/$profile/si-companion.exe" "$out/"
cp "$LLVM_MINGW/x86_64-w64-mingw32/bin/libunwind.dll" "$out/"
echo "$out/si-companion.exe"
