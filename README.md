# kache

```bash
./script/run.sh

Running go benchmark...
goos: darwin
goarch: arm64
pkg: anson627/kache
cpu: Apple M1 Max
BenchmarkEncode-10      60835383              1180 ns/op
BenchmarkDecode-10      56212864              1378 ns/op
PASS
ok      anson627/kache  152.052s
      155.70 real       190.89 user        17.15 sys
           207552512  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
               35244  page reclaims
                8543  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
              688073  signals received
              395334  voluntary context switches
             2448835  involuntary context switches
          3249633977  instructions retired
          1927032370  cycles elapsed
            21120192  peak memory footprint


/usr/bin/time -l cargo bench -- --measurement-time 60
Running rust benchmark...
    Finished `bench` profile [optimized] target(s) in 1.19s
     Running benches/benchmark.rs (target/release/deps/benchmark-05129b4cbbd5870a)
WARNING: HTML report generation will become a non-default optional feature in Criterion.rs 0.4.0.
This feature is being moved to cargo-criterion (https://github.com/bheisler/cargo-criterion) and will be optional in a future version of Criterion.rs. To silence this warning, either switch to cargo-criterion or enable the 'html_reports' feature in your Cargo.toml.

Gnuplot not found, using plotters backend
encode                  time:   [164.43 ns 164.89 ns 165.48 ns]
                        change: [-0.8699% -0.4767% -0.0130%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

decode                  time:   [757.74 ns 766.44 ns 776.25 ns]
                        change: [+1.6514% +2.3664% +3.1841%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe

      131.99 real       134.48 user         0.57 sys
            39206912  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                6833  page reclaims
                 187  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   1  signals received
                 116  voluntary context switches
               62654  involuntary context switches
           525564380  instructions retired
           330651513  cycles elapsed
            19940800  peak memory footprint
```