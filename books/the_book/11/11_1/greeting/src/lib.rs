pub fn greeting(name: &str) -> String {
    // format!("Hello {name}!") // passing function
    format!("Hello!") // failing function
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Joseph");
        assert!(result.contains("Joseph"),
    "\n Greeting did not contain name, value was '{result}'\n");
    }
}
