Scam is a chess engine written in Rust by Fabian von der Warth and Terje Kirstihagen. Scam is [uci](http://wbec-ridderkerk.nl/html/UCIProtocol.html/) compliant and supports [FRC](https://de.wikipedia.org/wiki/Chess960).

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
To run the `bench` command:
```
cargo run --release -- bench
```
To run the built-in benchmarks made by us to evaluate Scam's performance, run
```
cargo bench --bench movegen -- --verbose
```
, but make sure to close any other running processes first.
On Fabi's Intel i5-6400 this yields:
```
Gnuplot not found, using plotters backend
Benchmarking pseudolegal
Benchmarking pseudolegal: Warming up for 3.0000 s
Benchmarking pseudolegal: Collecting 100 samples in estimated 5.0011 s (1398850 iterations)
Benchmarking pseudolegal: Analyzing
pseudolegal             time:   [3.5781 us 3.5850 us 3.5936 us]
                        change: [-1.0866% -0.4203% +0.1589%] (p = 0.20 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
slope  [3.5781 us 3.5936 us] R^2            [0.9884607 0.9881747]
mean   [3.5830 us 3.6027 us] std. dev.      [25.700 ns 74.070 ns]
median [3.5733 us 3.5815 us] med. abs. dev. [10.029 ns 17.670 ns]

Benchmarking makemove
Benchmarking makemove: Warming up for 3.0000 s
Benchmarking makemove: Collecting 100 samples in estimated 5.0665 s (131300 iterations)
Benchmarking makemove: Analyzing
makemove                time:   [38.570 us 38.651 us 38.751 us]
                        change: [+23.488% +24.265% +25.203%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) high mild
  10 (10.00%) high severe
slope  [38.570 us 38.751 us] R^2            [0.9859528 0.9856462]
mean   [38.590 us 38.968 us] std. dev.      [300.02 ns 1.6091 us]
median [38.502 us 38.562 us] med. abs. dev. [77.118 ns 144.32 ns]

Benchmarking perft1
Benchmarking perft1: Warming up for 3.0000 s
Benchmarking perft1: Collecting 100 samples in estimated 5.1409 s (106050 iterations)
Benchmarking perft1: Analyzing
perft1                  time:   [48.164 us 48.280 us 48.424 us]
                        change: [+18.173% +18.934% +19.780%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) high mild
  10 (10.00%) high severe
slope  [48.164 us 48.424 us] R^2            [0.9814246 0.9810094]
mean   [48.246 us 48.770 us] std. dev.      [480.66 ns 2.1541 us]
median [48.081 us 48.174 us] med. abs. dev. [100.26 ns 189.77 ns]

Benchmarking perft2
Benchmarking perft2: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 8.5s, enable flat sampling, or reduce sample count to 50.
Benchmarking perft2: Collecting 100 samples in estimated 8.5363 s (5050 iterations)
Benchmarking perft2: Analyzing
perft2                  time:   [1.6921 ms 1.6950 ms 1.6984 ms]
                        change: [+17.217% +17.657% +18.088%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) high mild
  6 (6.00%) high severe
slope  [1.6921 ms 1.6984 ms] R^2            [0.9916868 0.9915719]
mean   [1.6930 ms 1.7014 ms] std. dev.      [11.811 us 29.448 us]
median [1.6884 ms 1.6913 ms] med. abs. dev. [4.0494 us 7.5664 us]
```