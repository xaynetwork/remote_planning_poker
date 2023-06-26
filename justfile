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
    #!/usr/bin/env sh
    set -euxo pipefail
    {{just_executable()}} dev-backend & \
    {{just_executable()}} dev-frontend & \
    wait

dev-backend:
    cargo watch -w common -w backend -x check -x 'run --bin backend'

dev-frontend:
    #!/usr/bin/env sh
    cd frontend
    trunk serve --public-url /

fmt *args:
    cargo +nightly fmt --all -- {{ args }}; \
    cargo sort --grouped --workspace {{ args }}

check: 
    cargo clippy --all-targets --locked

build:
    {{just_executable()}} build-frontend
    {{just_executable()}} build-backend

build-backend:
    cargo build --locked --release --bin backend

build-frontend:
    #!/usr/bin/env sh
    cd frontend
    trunk build
