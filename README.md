# rust-postgres
Small app to read Postgres data with a rust app


## Adding dependencies to Cargo.toml

``` rust
[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-postgres = "0.7"
```

## Write your code

Create a file for your source code
```
touch src/main.rs
```

Write down your code here.

## Build and execute

``` rust
cargo build
cargo run
```

## Expected result

```
vscode ➜ /workspaces/rust-postgres (main) $ cargo build && cargo run
   Compiling rust-postgres v0.1.0 (/workspaces/rust-postgres)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.89s
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/rust-postgres`
id: 1, name: Donald Duck
id: 2, name: Daisy Duck
id: 3, name: Scrooge McDuck
```