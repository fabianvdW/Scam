Scam is a chess engine written in Rust by Fabian von der Warth and Terje Kirstihagen.

## Compiling Scam
Install the rust compiler and cargo, the recommended way to do this is via [rustup](https://rustup.rs/). Then run
```
git clone https://github.com/fabianvdW/Scam
cd Scam
set RUSTFLAGS=-C target-cpu=native
cargo run --release
```
If you're on an AMD processor with bmi2 instruction available but microcoded, please disable them manually:
```
set RUSTFLAGS=-C target-cpu=native -C target-feature=-bmi2
cargo run --release
```
## Cross-compiling Scam
### From Windows to Linux
Install Rust nightly:
```
rustup toolchain install nightly
```
Add `.cargo` folder, `.cargo/config` file in the root folder of the repo. Content:
```
[target.x86_64-unknown-linux-musl]
linker="rust-lld"
```
Finally we are ready to install our cross-compilation target and use it:
```
rustup default nightly
rustup target add x86_64-unknown-linux-musl
set RUSTFLAGS=-C target-cpu=<your-target-machine-cpu>
cargo rustc --release --bin scam --target x86_64-unknown-linux-musl
```
To find a suitable target-cpu, choose one from
```
rustc --target=x86_64-unknown-linux-musl --print target-cpus
```
Please note the comment in our [build.rs file](https://github.com/fabianvdW/Scam/blob/8249bb32162a5d721789a9d3020cae48fb95051c/src/build.rs#L17):
```
In the case that the Host does not have BMI2, while the target-cpu wants BMI2 instructions, this build script will fail.
```
## Benchmarking Scam
To run the built-in benchmarks made by us to evaluate Scam's performance, run
```
cargo bench --bench movegen -- --verbose
```
, but make sure to close any other running processes first.
On Fabi's Intel i5-6400 this yields: #To be filled, remove this
```

```