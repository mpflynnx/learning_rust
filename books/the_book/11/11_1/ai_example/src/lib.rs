// 1. Define a simple Error type for the test.
//    In real-world scenarios, this would be a more detailed error.
#[derive(Debug)]
pub struct TestError;

// Implement conversion from a standard error (like I/O) 
// if you were calling functions that return std::io::Error.
// For this simple example, we'll focus on a custom function.

// 2. Define a function we want to test that returns a Result.

fn potentially_failing_function(input: i32) -> Result<i32, TestError> {
    if input > 10 {
        Ok(input * 2)
    } else {
        // Return an Err, which the '?' operator will propagate
        Err(TestError)
    }
}

// 3. The test function signature is key: it returns a Result<(), TestError>
//    instead of the usual '()'. The '?' operator is now available.
#[test]
fn test_successful_operation() -> Result<(), TestError> {
    // Call the function under test.
    // If 'potentially_failing_function' returns Err(TestError), 
    // the '?' operator immediately returns Err(TestError) from 
    // the 'test_successful_operation' function, causing the test to FAIL.
    let result = potentially_failing_function(15)?; // ✅ Success, result = 30

    // Assert the expected behavior on success
    assert_eq!(result, 30);

    // If all assertions pass and no errors were propagated, the test passes.
    Ok(()) 
}
