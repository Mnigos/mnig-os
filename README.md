# mnig-os

A tiny Rust OS learning project.

The first goal is intentionally small:

```text
Boot an AArch64 kernel in QEMU and print text through UART.
```

This repository currently starts as a normal Rust scaffold so the formatter, editor integration, and repository hygiene are in place before adding `no_std`, a target spec, linker script, and QEMU boot flow.

## Local checks

```sh
cargo fmt --check
cargo clippy
cargo test
```

The same checks are also available through package scripts:

```sh
bun run format
bun run check
bun run test
```
