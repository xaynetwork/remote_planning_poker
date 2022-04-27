default:
  just --list

install-deps:
  @echo 'Installing dependencies...'
  rustup target add wasm32-unknown-unknown
  cargo install \
    trunk \
    wasm-bindgen-cli \
    cargo-watch
  npm install -g tailwindcss

dev:
  #!/usr/bin/env bash
  set -euxo pipefail
  {{just_executable()}} dev-backed & \
  {{just_executable()}} dev-frontend & \
  wait

dev-backed:
  cd backend; cargo watch -x run

dev-frontend:
  cd frontend; trunk serve
