# Actix Web Demo

## Build

```shell
cd ~/ws_rust
cargo new hello-actix-web
cd hello-actix-web
```

update Cargo.toml
```toml
[dependencies]
actix-web = "4"
env_logger = "0.11.6"  # 用于日志记录
```

## Launch

```shell
cargo check
cargo run
```

## Todo

- [Writing an Application](https://actix.rs/docs/application)

## REF

- [official api docs](https://actix.rs/docs/getting-started/)
- [Discord 为什么从 Go 迁移到 Rust](https://discord.com/blog/why-discord-is-switching-from-go-to-rust)
