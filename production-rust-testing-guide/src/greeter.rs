/// A small collaboration boundary used to demonstrate mocking.
///
/// `cfg_attr(test, mockall::automock)` generates `MockGreeter` only for unit
/// tests. Production builds do not depend on mock types.
#[cfg_attr(test, mockall::automock)]
pub trait Greeter {
    fn greet(&self, name: &str) -> String;
}

pub struct EnglishGreeter;

impl Greeter for EnglishGreeter {
    fn greet(&self, name: &str) -> String {
        format!("Hello, {name}")
    }
}

pub fn render_welcome<G: Greeter>(greeter: &G, name: &str) -> String {
    greeter.greet(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;

    #[test]
    fn english_greeter_renders_welcome_message() {
        let greeter = EnglishGreeter;
        assert_eq!(render_welcome(&greeter, "Alice"), "Hello, Alice");
    }

    #[test]
    fn greeting_calls_dependency_once_with_automock() {
        let mut mock = MockGreeter::new();

        mock.expect_greet()
            .with(eq("Alice"))
            .times(1)
            .returning(|_| "Hello, Alice".to_owned());

        assert_eq!(render_welcome(&mock, "Alice"), "Hello, Alice");
    }
}
