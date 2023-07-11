# Go plugin

[Go](https://go.dev/) WASM plugin for [proto](https://github.com/moonrepo/proto).

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
