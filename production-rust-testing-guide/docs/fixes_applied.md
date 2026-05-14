# Fixes Applied to the Production Rust Testing Guide Examples

This repository fixes the three compileability issues reported in the original guide examples.

## 1. `#[should_panic]` and `10 / 0`

Problematic example:

```rust
#[test]
#[should_panic]
fn division_by_zero_panics() {
    let _ = 10 / 0;
}
```

Why it is wrong:

- Literal division by zero is caught by Rust's `unconditional_panic` lint.
- The example is not a good runtime panic test.

Replacement:

```rust
#[test]
#[should_panic(expected = "division by zero is not allowed")]
fn divide_or_panic_rejects_zero_divisor() {
    divide_or_panic(10, 0);
}
```

The panic now comes from an explicit runtime contract check.

## 2. `i32::ANY` in proptest

Problematic example:

```rust
vec(i32::ANY, 1..100)
```

Why it is wrong:

- `i32::ANY` is not an associated constant on Rust's primitive `i32` type.

Replacement:

```rust
vec(any::<i32>(), 1..100)
```

This uses `proptest::prelude::*` and the standard `any::<T>()` strategy.

## 3. Invalid `mockall::mock!` invocation

Problematic example:

```rust
mock! {
    Greeter,
    fn greet(&self, name: &str) -> String;
}
```

Why it is wrong:

- Current `mockall::mock!` syntax requires a mock struct definition.
- To mock a trait, implement the trait for the mock struct inside the macro.

Replacement:

```rust
mock! {
    pub GreeterDouble {}

    impl Greeter for GreeterDouble {
        fn greet(&self, name: &str) -> String;
    }
}
```

This repository also includes the simpler `#[automock]` style because it is easier for most users.
