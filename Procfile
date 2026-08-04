api: API_HOST=0.0.0.0 API_PORT=8000 RUST_LOG=info,actix=info cargo run --manifest-path news_service/Cargo.toml --bin api
cron: CRON_HOST=0.0.0.0 CRON_PORT=8001 RUST_LOG=info,actix=info cargo run --manifest-path news_service/Cargo.toml --bin cron
web: cd web && npm run dev