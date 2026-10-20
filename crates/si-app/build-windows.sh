#!/usr/bin/env bash
# Build si-companion.exe for Windows from WSL (native app spec §2) and put it,
# with the runtime DLL it needs, in target/si-companion/.
#
#   crates/si-app/build-windows.sh            # release
#   crates/si-app/build-windows.sh --debug
#
# Needs the Rust target x86_64-pc-windows-gnullvm and llvm-mingw. Set
# LLVM_MINGW to its folder if it isn't the one below.
set -euo pipefail
cd "$(dirname "$0")/../.."
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
# A new file, not an overwrite: Windows keeps running an open exe's old
# image for every new launch of the same file.
rm -f "$out/si-companion.exe"
cp "target/$TARGET/$profile/si-companion.exe" "$out/"
cp "$LLVM_MINGW/x86_64-w64-mingw32/bin/libunwind.dll" "$out/"
# A launcher that copies each build to its own file under the app's data
# folder before starting it, so a running old build never pins the new one.
build_id="$(date +%Y%m%d-%H%M%S)"
cat > "$out/run-companion.cmd" <<CMD
@echo off
set "bin=%LOCALAPPDATA%\\si-companion\\bin"
if not exist "%bin%" mkdir "%bin%"
copy /y "%~dp0si-companion.exe" "%bin%\\si-companion-$build_id.exe" >nul
copy /y "%~dp0libunwind.dll" "%bin%\\libunwind.dll" >nul
start "" "%bin%\\si-companion-$build_id.exe" %*
CMD
echo "$out/si-companion.exe  (launch with $out/run-companion.cmd)"
# A running instance keeps the old image alive for every launch from this
# path, new file or not: it has to be closed before the new build shows.
if tasklist.exe 2>/dev/null | grep -qi "si-companion.exe"; then
  echo "WARNING: si-companion.exe is running; close it, or every launch from this path keeps the old build" >&2
fi
