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

## RustRover IDE

cargo check和 cargo run等命令都可以设置configuration，以便在IDE上快速调试。

## Todo

- [Writing an Application](https://actix.rs/docs/application)
- [Actix 实战快速入门 Rust](https://knots.l0u0l.com/Rust/%E5%85%A5%E9%97%A8%E7%AC%94%E8%AE%B0/Actix%20Web.html)

## REF

- [official api docs](https://actix.rs/docs/getting-started/)
- [Discord 为什么从 Go 迁移到 Rust](https://discord.com/blog/why-discord-is-switching-from-go-to-rust)
