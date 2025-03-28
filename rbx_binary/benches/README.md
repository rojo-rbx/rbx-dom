# `rbx_binary` Benchmark Suite

This directory contains a suite of benchmarks used to measure the performance of the `rbx_binary` serializer and deserializer.

### Adding a new benchmark

To add a new benchmark, first add the file you'd like to measure performance against to the `files` directory. Then, add a new benchmark function to `suite/main.rs`, like this:
```rust
pub fn my_benchmark(c: &mut Criterion) {
    bench(
        &mut c.benchmark_group("My Benchmark")
        include_bytes!("../files/bench-file.rbxl"),
    )
}
```
and also make sure to add your benchmark function to the `criterion_group!` macro invocation.

Benchmark groups provide a number of configuration options which are useful under different circumstances. See the [Criterion.rs benchmark configuration documentation](https://bheisler.github.io/criterion.rs/book/user_guide/advanced_configuration.html) for details.

### Running the benchmarks

To run all benchmarks, run the following command somewhere in the `rbx_binary` crate directory:
```bash
cargo bench
```

To run a specific benchmark, run the following command somewhere in the `rbx_binary` crate directory, subsituting `My Benchmark` with the name of the benchmark group:
```bash
cargo bench "My Benchmark"
```

To measure only serialization or deserialization, add `/Serialize` or `/Deserialize` to the end of the benchmark name, like this:
```bash
cargo bench "My Benchmark/Serialize"
```

Once the benchmark is complete, an HTML report will be generated at `rbx-dom/target/criterion/reports/index.html` that contains detailed statistics collected during the run. This file can be opened in a web browser.

For more information, see the [Criterion.rs documentation](https://bheisler.github.io/criterion.rs/book/).
