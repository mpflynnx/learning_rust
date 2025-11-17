## Chapter 11. Writing Automated Tests

###  External References
- [Chapter 11 | Testing](https://doc.rust-lang.org/stable/book/ch11-00-testing.html)
- [Chapter 11_1 | Writing Automated Tests](/11/11_1/)
- [Chapter 11_2 | Controlling How Tests Run](/11/11_2/)
- [Chapter 11_3 | Test Organization](/11/11_3/)


- [Quiz](https://gemini.google.com/share/2b90fb7fa121)

- [Study Guide](https://docs.google.com/document/d/1GjKxsLyRkHE3B6iKgujNF4Pi-EEfwuFkSBRCRgS8TqM/edit?tab=t.0)


### cargo new commands

```bash
# Create new library with default test functions
$ cargo new add --lib
```

### cargo test commands

```bash
# Run all tests whether #[ignored] or not
$ cargo test -- --include-ignored
# Run only #[ignored] tests 
$ cargo test -- --ignored
# Run all tests
$ cargo test
# Run all tests with 'add' in name
$ cargo test add
# Run only test named 'one_hundred' test
$ cargo test one_hundred
# Show all test println! outputs
$ cargo test -- --show-output
# Run only tests defined in tests/integration_test.rs
$ cargo test --test integration_test
# Run tests sequentially (not in parallel)
$ cargo test --test-threads=1
```
