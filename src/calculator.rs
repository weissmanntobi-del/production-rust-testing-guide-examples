use std::fmt;

/// Errors returned by safe arithmetic helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivideError {
    DivisionByZero,
}

impl fmt::Display for DivideError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DivideError::DivisionByZero => write!(f, "division by zero is not allowed"),
        }
    }
}

impl std::error::Error for DivideError {}

/// Production-friendly API: prefer `Result` when invalid input is expected.
pub fn checked_divide(a: i32, b: i32) -> Result<i32, DivideError> {
    if b == 0 {
        Err(DivideError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

/// Panic-based API: useful only when panic is part of the explicit contract.
///
/// This avoids the invalid `let _ = 10 / 0;` example. The panic comes from
/// a deliberate runtime contract check, not from a literal compile-time division
/// by zero.
#[track_caller]
pub fn divide_or_panic(a: i32, b: i32) -> i32 {
    checked_divide(a, b).expect("division by zero is not allowed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_divide_returns_value_for_valid_input() {
        assert_eq!(checked_divide(10, 2), Ok(5));
    }

    #[test]
    fn checked_divide_returns_error_for_zero_divisor() {
        assert_eq!(checked_divide(10, 0), Err(DivideError::DivisionByZero));
    }

    #[test]
    #[should_panic(expected = "division by zero is not allowed")]
    fn divide_or_panic_rejects_zero_divisor() {
        divide_or_panic(10, 0);
    }
}
