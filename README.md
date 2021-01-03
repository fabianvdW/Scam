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
Benchmarking pseudolegal
Benchmarking pseudolegal: Warming up for 3.0000 s
Benchmarking pseudolegal: Collecting 100 samples in estimated 5.0142 s (1378650 iterations)
Benchmarking pseudolegal: Analyzing
pseudolegal             time:   [3.6421 us 3.6609 us 3.6858 us]
                        change: [-0.4971% -0.1544% +0.2386%] (p = 0.46 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe
slope  [3.6421 us 3.6858 us] R^2            [0.9283395 0.9259509]
mean   [3.6411 us 3.6663 us] std. dev.      [24.004 ns 99.125 ns]
median [3.6338 us 3.6409 us] med. abs. dev. [8.9408 ns 14.766 ns]

Benchmarking makemove
Benchmarking makemove: Warming up for 3.0000 s
Benchmarking makemove: Collecting 100 samples in estimated 5.0445 s (146450 iterations)
Benchmarking makemove: Analyzing
makemove                time:   [34.501 us 34.523 us 34.547 us]
                        change: [-0.1189% +0.0499% +0.2307%] (p = 0.59 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
slope  [34.501 us 34.547 us] R^2            [0.9986416 0.9986313]
mean   [34.528 us 34.615 us] std. dev.      [145.38 ns 289.36 ns]
median [34.482 us 34.542 us] med. abs. dev. [82.922 ns 141.59 ns]

Benchmarking perft1
Benchmarking perft1: Warming up for 3.0000 s
Benchmarking perft1: Collecting 100 samples in estimated 5.2242 s (116150 iterations)
Benchmarking perft1: Analyzing
perft1                  time:   [45.010 us 45.272 us 45.581 us]
                        change: [+0.1570% +0.5388% +0.9781%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe
slope  [45.010 us 45.581 us] R^2            [0.9195911 0.9180176]
mean   [44.936 us 45.269 us] std. dev.      [458.12 ns 1.1616 us]
median [44.811 us 44.883 us] med. abs. dev. [111.24 ns 200.81 ns]

Benchmarking perft2
Benchmarking perft2: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 8.0s, enable flat sampling, or reduce sample count to 50.
Benchmarking perft2: Collecting 100 samples in estimated 8.0336 s (5050 iterations)
Benchmarking perft2: Analyzing
perft2                  time:   [1.5692 ms 1.5704 ms 1.5720 ms]
                        change: [-1.0765% -0.8903% -0.6971%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe
slope  [1.5692 ms 1.5720 ms] R^2            [0.9975675 0.9975081]
mean   [1.5691 ms 1.5729 ms] std. dev.      [4.3179 us 14.883 us]
median [1.5678 ms 1.5693 ms] med. abs. dev. [2.1564 us 3.6131 us]
```