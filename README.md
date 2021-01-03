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
Benchmarking pseudolegal: Collecting 100 samples in estimated 5.0012 s (1363500 iterations)
Benchmarking pseudolegal: Analyzing
pseudolegal             time:   [3.6580 us 3.6618 us 3.6658 us]
                        change: [+0.3150% +0.6701% +1.1231%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe
slope  [3.6580 us 3.6658 us] R^2            [0.9970286 0.9970084]
mean   [3.6618 us 3.6845 us] std. dev.      [17.876 ns 97.107 ns]
median [3.6574 us 3.6637 us] med. abs. dev. [12.162 ns 20.840 ns]

Benchmarking makemove
Benchmarking makemove: Warming up for 3.0000 s
Benchmarking makemove: Collecting 100 samples in estimated 5.1912 s (136350 iterations)
Benchmarking makemove: Analyzing
makemove                time:   [37.614 us 37.641 us 37.669 us]
                        change: [-0.8519% -0.4057% -0.0076%] (p = 0.04 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
slope  [37.614 us 37.669 us] R^2            [0.9985878 0.9985804]
mean   [37.633 us 37.835 us] std. dev.      [145.76 ns 869.45 ns]
median [37.575 us 37.619 us] med. abs. dev. [63.602 ns 134.55 ns]

Benchmarking perft1
Benchmarking perft1: Warming up for 3.0000 s
Benchmarking perft1: Collecting 100 samples in estimated 5.0540 s (106050 iterations)
Benchmarking perft1: Analyzing
perft1                  time:   [47.477 us 47.545 us 47.658 us]
                        change: [-1.0068% -0.3256% +0.2821%] (p = 0.37 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
slope  [47.477 us 47.658 us] R^2            [0.9932153 0.9927310]
mean   [47.492 us 47.815 us] std. dev.      [161.03 ns 1.4566 us]
median [47.429 us 47.472 us] med. abs. dev. [64.994 ns 138.32 ns]

Benchmarking perft2
Benchmarking perft2: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 8.7s, enable flat sampling, or reduce sample count to 50.
Benchmarking perft2: Collecting 100 samples in estimated 8.6705 s (5050 iterations)
Benchmarking perft2: Analyzing
perft2                  time:   [1.6789 ms 1.6798 ms 1.6809 ms]
                        change: [-0.4075% -0.1288% +0.1635%] (p = 0.41 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
slope  [1.6789 ms 1.6809 ms] R^2            [0.9990818 0.9990765]
mean   [1.6795 ms 1.6870 ms] std. dev.      [5.3730 us 30.030 us]
median [1.6772 ms 1.6789 ms] med. abs. dev. [2.4701 us 4.9788 us]
```