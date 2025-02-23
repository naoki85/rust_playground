# greet-user

## Prepare

```
$ cd ../greet
$ rustup target add wasm32-unknown-unknown
$ cargo component build --target wasm32-unknown-unknown
```

## Run

```
$ cargo run ../greet/target/wasm32-unknown-unknown/debug/greet.wasm
```