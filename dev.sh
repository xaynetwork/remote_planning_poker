#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

# for this to work we need to build the tailwind file at least once

pushd frontend
tailwindcss -c tailwind.config.js -o tailwind.css
popd


(trap 'kill 0' SIGINT; \
  bash -c 'cd api; cargo watch -x run' & \
  bash -c 'cd frontend; trunk serve' & \
  bash -c 'cd frontend; tailwindcss -c tailwind.config.js -o tailwind.css --watch')