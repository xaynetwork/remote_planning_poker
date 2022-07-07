# Remote planning poker

This is a simple estimation tool for remote teams. It is an example of full-stack Rust web application using [Yew](https://yew.rs/) on the frontend and [Axum](https://github.com/tokio-rs/axum) on the backend.

## Crates

- `frontend`: Yew web app.
- `backend`: Axum backend exposing websocket api.
- `common`: Common types and logic shared by frontend/backend.

## Development

You need to have Rust and Node.js (for tailwindcss support) installed on your machine.

It is recommended to use `just` task runner for better DX. You can find information how to install it in [just repo](https://github.com/casey/just#packages).

To install needed dependencies simply run:

```bash
just install-deps
```

To start development simply run dev task, which will spin up [Trunk](https://trunkrs.dev/) development server for Yew, and cargo watch for backend:

```bash
just dev
```

## License

Apache-2.0
