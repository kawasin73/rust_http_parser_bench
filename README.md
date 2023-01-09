# rust http parser bench

The fastest HTTP parser of Rust is `picohttpparser_sys` which is Rust binding of
C library though.

The fastest pure Rust HTTP parser is `thhp`.

```
$ cargo +nightly bench
   Compiling rust_http_parser_bench v0.1.0 (/Users/kawasin73/src/github.com/kawasin73/rust_http_parser_bench)
    Finished bench [optimized] target(s) in 1.68s
warning: the following packages contain code that will be rejected by a future version of Rust: traitobject v0.1.0
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 76`
     Running unittests src/main.rs (target/release/deps/rust_http_parser_bench-63366b14855362d6)

running 16 tests
test tests::bench_dumb_http_parser   ... bench:       2,012 ns/iter (+/- 174) = 349 MB/s
test tests::bench_http_box           ... bench:         645 ns/iter (+/- 73) = 1089 MB/s
test tests::bench_http_bytes         ... bench:       2,288 ns/iter (+/- 311) = 307 MB/s
test tests::bench_http_muncher       ... bench:         876 ns/iter (+/- 100) = 802 MB/s
test tests::bench_http_parser        ... bench:       3,889 ns/iter (+/- 378) = 180 MB/s
test tests::bench_http_pull_parser   ... bench:       7,616 ns/iter (+/- 855) = 92 MB/s
test tests::bench_http_tiny          ... bench:      14,301 ns/iter (+/- 1,454) = 49 MB/s
test tests::bench_httparse           ... bench:         298 ns/iter (+/- 36) = 2359 MB/s
test tests::bench_milstian_http      ... bench:      25,205 ns/iter (+/- 2,708) = 27 MB/s
test tests::bench_picohttpparser_sys ... bench:         143 ns/iter (+/- 11) = 4916 MB/s
test tests::bench_rhymuweb           ... bench:       6,789 ns/iter (+/- 1,055) = 103 MB/s
test tests::bench_rocket_http_hyper  ... bench:       4,255 ns/iter (+/- 527) = 165 MB/s
test tests::bench_saf_httparser      ... bench:       6,293 ns/iter (+/- 784) = 111 MB/s
test tests::bench_stream_httparse    ... bench:       1,947 ns/iter (+/- 194) = 361 MB/s
test tests::bench_thhp               ... bench:         237 ns/iter (+/- 34) = 2966 MB/s
test tests::bench_uhttp_request      ... bench:         274 ns/iter (+/- 31) = 2565 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 16 measured; 0 filtered out; finished in 48.00s
```

```
$ RUSTFLAGS="-C target-feature=+sse4.2" cargo +nightly bench
    Finished bench [optimized] target(s) in 0.28s
warning: the following packages contain code that will be rejected by a future version of Rust: traitobject v0.1.0
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 78`
     Running unittests src/main.rs (target/release/deps/rust_http_parser_bench-63366b14855362d6)

running 16 tests
test tests::bench_dumb_http_parser   ... bench:       2,085 ns/iter (+/- 537) = 337 MB/s
test tests::bench_http_box           ... bench:         592 ns/iter (+/- 43) = 1187 MB/s
test tests::bench_http_bytes         ... bench:       2,347 ns/iter (+/- 341) = 299 MB/s
test tests::bench_http_muncher       ... bench:         795 ns/iter (+/- 141) = 884 MB/s
test tests::bench_http_parser        ... bench:       3,791 ns/iter (+/- 594) = 185 MB/s
test tests::bench_http_pull_parser   ... bench:       6,990 ns/iter (+/- 658) = 100 MB/s
test tests::bench_http_tiny          ... bench:      12,675 ns/iter (+/- 1,219) = 55 MB/s
test tests::bench_httparse           ... bench:         276 ns/iter (+/- 11) = 2547 MB/s
test tests::bench_milstian_http      ... bench:      23,279 ns/iter (+/- 2,616) = 30 MB/s
test tests::bench_picohttpparser_sys ... bench:         135 ns/iter (+/- 24) = 5207 MB/s
test tests::bench_rhymuweb           ... bench:       6,868 ns/iter (+/- 631) = 102 MB/s
test tests::bench_rocket_http_hyper  ... bench:       4,119 ns/iter (+/- 420) = 170 MB/s
test tests::bench_saf_httparser      ... bench:       5,870 ns/iter (+/- 528) = 119 MB/s
test tests::bench_stream_httparse    ... bench:       1,781 ns/iter (+/- 202) = 394 MB/s
test tests::bench_thhp               ... bench:         177 ns/iter (+/- 14) = 3971 MB/s
test tests::bench_uhttp_request      ... bench:         261 ns/iter (+/- 24) = 2693 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 16 measured; 0 filtered out; finished in 19.03s
```

## Environment

* Macbook Pro (13-inch, 2018, Four Thunderbolt 3 Ports)
* Processor: 2.7 GHz Quad-Core Intel Core i7
* Memory: 16 GB 2133 MHz LPDDR3

```
$ cargo -V
cargo 1.68.0-nightly (8c460b223 2023-01-04)
```

## LICENSE

MIT
