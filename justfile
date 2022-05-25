default:
  just --list

clean:
  rm -rf frontend/dist frontend/tailwind.css target
  cargo clean

install-deps:
  @echo 'Installing dependencies...'
  rustup target add wasm32-unknown-unknown
  cargo install --locked trunk
  cargo install wasm-bindgen-cli
  cargo install cargo-watch
  npm install -g tailwindcss

dev:
  #!/usr/bin/env bash
  set -euxo pipefail
  {{just_executable()}} dev-backed & \
  {{just_executable()}} dev-frontend & \
  wait

dev-backed:
  cargo watch -w common -w backend -x check -x 'run --bin backend'

dev-frontend:
  cd frontend; trunk serve

build:
  {{just_executable()}} build-frontend
  {{just_executable()}} build-backend

build-backend:
  cargo build --locked --release --bin backend

build-frontend:
  cd frontend; trunk build
