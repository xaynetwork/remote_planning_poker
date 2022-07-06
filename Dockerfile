FROM debian:bookworm-slim

WORKDIR /service
COPY ./target/release/backend /service
COPY ./frontend/dist /service/dist

RUN chmod +x ./backend

CMD ["./backend"]
