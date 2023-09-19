# Go plugin

[Go](https://go.dev/) WASM plugin for [proto](https://github.com/moonrepo/proto).

```shell
proto install go
```

## Post-install hook

After installation, we'll inject a `GOBIN` environment variable into your shell, pointing to
`~/go/bin`, if it does not already exist. This variable will be used to locate Go binaries across
all installed versions. This functionality can be skipped by passing `--no-gobin` during
installation.

```shell
proto install go -- --no-gobin
```

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasi
```

Test the plugin by running `proto` commands. Requires proto >= v0.12.

```shell
proto install go-test
proto list-remote go-test
```
