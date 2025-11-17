pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

// private by default
fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {

    // Bring all parents into scope (including private)
    use super::*;

    #[test]
    fn adds_three_to_two(){
        let result = add_two(3); // Uses public function
        assert_eq!(result, 5);
    }

    #[test]
    fn internal() {
        let result = internal_adder(2, 2); // Uses private function
        assert_eq!(result, 4);
    }
}
