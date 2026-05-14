/// Reverse a string by Unicode scalar values.
///
/// This is intentionally simple: the testing examples focus on properties,
/// not on user-facing grapheme-cluster handling.
pub fn reverse_chars(input: &str) -> String {
    input.chars().rev().collect()
}

/// Reverse a slice into a new vector.
pub fn reverse_vec<T: Clone>(values: &[T]) -> Vec<T> {
    values.iter().rev().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::collection::vec;
    use proptest::prelude::*;

    #[test]
    fn reverse_chars_reverses_ascii_text() {
        assert_eq!(reverse_chars("rust"), "tsur");
    }

    #[test]
    fn reverse_vec_reverses_numbers() {
        assert_eq!(reverse_vec(&[1, 2, 3]), vec![3, 2, 1]);
    }

    proptest! {
        #[test]
        fn reversing_string_twice_is_identity(s in any::<String>()) {
            let reversed = reverse_chars(&s);
            let back = reverse_chars(&reversed);
            prop_assert_eq!(s, back);
        }

        #[test]
        fn reversing_list_twice_is_identity(lst in vec(any::<i32>(), 1..100)) {
            let reversed = reverse_vec(&lst);
            let back = reverse_vec(&reversed);
            prop_assert_eq!(lst, back);
        }
    }
}
