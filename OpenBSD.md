# Installation on OpenBSD

In case of LibreSSL version mismatch, update the dependencies, e.g. currently have been modified with:

```sh
cargo update -p openssl-sys --precise 0.9.110
cargo update -p openssl --precise 0.10.74
```

Install from the root of the repo:

```sh
cargo install --path . --locked
```

