# Production Rust Testing Guide — Compilable Examples

This repository contains corrected, copy/paste-friendly Rust examples for a production testing guide.

It specifically fixes three reported problems:

1. A `#[should_panic]` example using `10 / 0`.
2. A `proptest` example using the invalid `i32::ANY` constant.
3. An invalid `mockall::mock!` invocation.

## What this repo demonstrates

- Unit tests close to the implementation
- Integration tests in `tests/`
- A valid `#[should_panic]` example
- Property-based tests with `proptest`
- Mocking with `mockall` using both `#[automock]` and `mock!`
- Criterion benchmarking
- GitHub Actions CI for fmt, clippy, tests, and benchmarks compilation

## Project structure

```text
production-rust-testing-guide-examples/
  src/
    lib.rs
    calculator.rs
    reverse.rs
    greeter.rs
  tests/
    should_panic_example.rs
    property_tests.rs
    mockall_automock.rs
    mockall_manual_macro.rs
    integration_smoke.rs
  benches/
    fibonacci_bench.rs
  docs/
    fixes_applied.md
  scripts/
    local_check.sh
  .github/workflows/ci.yml
  Cargo.toml
  rust-toolchain.toml
```

## Run locally

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
cargo bench --no-run
```

Or run everything:

```bash
./scripts/local_check.sh
```

## Corrected snippets

### Valid `#[should_panic]` example

```rust
#[test]
#[should_panic(expected = "division by zero is not allowed")]
fn should_panic_example_uses_runtime_contract_check() {
    divide_or_panic(10, 0);
}
```

### Valid `proptest` vector strategy

```rust
proptest! {
    #[test]
    fn reversing_list_twice_is_identity(lst in vec(any::<i32>(), 1..100)) {
        let reversed = reverse_vec(&lst);
        let back = reverse_vec(&reversed);
        prop_assert_eq!(lst, back);
    }
}
```

### Valid `mockall::mock!` syntax

```rust
mock! {
    pub GreeterDouble {}

    impl Greeter for GreeterDouble {
        fn greet(&self, name: &str) -> String;
    }
}
```

## GitHub upload steps

```bash
git init
git add .
git commit -m "Add corrected Rust testing guide examples"
git branch -M main
git remote add origin https://github.com/weissmanntobi-del/production-rust-testing-guide-examples.git
git push -u origin main
```

## License

Licensed from MIT.
