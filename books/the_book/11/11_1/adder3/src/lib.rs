pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    // Use Result<T, E> in Tests
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);
        
        if result == 5 {
            Ok(())
        } else {
            Err(String::from("result: {result} not 4"))
        }
    }
}
