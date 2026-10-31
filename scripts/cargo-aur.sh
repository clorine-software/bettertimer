#!/bin/sh
set -e
docker run --rm -it -v "$PWD":/src -w /src archlinux:latest /bin/bash -c "
  pacman -Sy --noconfirm base-devel rust cargo
  cargo install cargo-aur
  cargo clean
  cargo aur
"
