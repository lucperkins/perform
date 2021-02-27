check:
    cargo check

clippy:
    cargo clippy -- --deny warnings

fmt:
    cargo fmt --all -- --check

test:
    cargo test

ci: check fmt clippy test