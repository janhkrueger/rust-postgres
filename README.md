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


## Further optimisation

Because Rust apps tend to be quite large, we can add some optimisations for the release build.


Add the following to the Cargo.toml:

```
[profile.release]
opt-level = "s"
lto = true      # Enable Link Time Optimization
codegen-units = 1 # Reduce codegen units for better optimization
panic = "abort" # Use abort for panics to reduce binary size
```


### opt-level

Explanation from the docs:

```
opt-level

The opt-level setting controls the -C opt-level flag which controls the level of optimization. Higher optimization levels may produce faster runtime code at the expense of longer compiler times. Higher levels may also change and rearrange the compiled code which may make it harder to use with a debugger.

The valid options are:

0: no optimizations
1: basic optimizations
2: some optimizations
3: all optimizations
"s": optimize for binary size
"z": optimize for binary size, but also turn off loop vectorization.
```

I have choosen option **"s"** because it works for me. Please be free to experiment with the options in your setup.


### lto

```
lto

The lto setting controls rustc’s -C lto, -C linker-plugin-lto, and -C embed-bitcode options, which control LLVM’s link time optimizations. LTO can produce better optimized code, using whole-program analysis, at the cost of longer linking time.

The valid options are:

false: Performs “thin local LTO” which performs “thin” LTO on the local crate only across its codegen units. No LTO is performed if codegen units is 1 or opt-level is 0.
true or "fat": Performs “fat” LTO which attempts to perform optimizations across all crates within the dependency graph.
"thin": Performs “thin” LTO. This is similar to “fat”, but takes substantially less time to run while still achieving performance gains similar to “fat”.
"off": Disables LTO.
```

Choosen option for me: "**true**". Again, feel free to look at the option what works for you.


### codegen-units

```
codegen-units

The codegen-units setting controls the -C codegen-units flag which controls how many “code generation units” a crate will be split into. More code generation units allows more of a crate to be processed in parallel possibly reducing compile time, but may produce slower code.

This option takes an integer greater than 0.

The default is 256 for incremental builds, and 16 for non-incremental builds.
```

Choosen option for me: **1**
Slow compile times are fine for me, I want small, fast code.


> "Build once, run many".


### panic

````
panic

The panic setting controls the -C panic flag which controls which panic strategy to use.

The valid options are:

"unwind": Unwind the stack upon panic.
"abort": Terminate the process upon panic.
When set to "unwind", the actual value depends on the default of the target platform. For example, the NVPTX platform does not support unwinding, so it always uses "abort".

Tests, benchmarks, build scripts, and proc macros ignore the panic setting. The rustc test harness currently requires unwind behavior. See the panic-abort-tests unstable flag which enables abort behavior.

Additionally, when using the abort strategy and building a test, all of the dependencies will also be forced to build with the unwind strategy.
````

Choosen option: **abort**.

### More here:

https://doc.rust-lang.org/cargo/reference/profiles.html