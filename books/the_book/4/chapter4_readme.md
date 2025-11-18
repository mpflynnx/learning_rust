## Chapter 4. Understanding Ownership

### Ownership rules
1. Each value in Rust has an owner
1. There can only be one owner at a time
1. When a owner goes out of scope, the value will be dropped

### Reference rules
1. At any given time, you can have either one mutable reference or any number of immutable references.
1. References must always be valid.

### External References
- [AI quiz](https://gemini.google.com/share/d3eb1411b5f1)
- [AI generated quiz](https://gemini.google.com/share/70396f79bd54)
- [Flashcards](https://gemini.google.com/share/f775d7ef227e)
- [Study guide](https://docs.google.com/document/d/1Scv-MM06__p_eTuw9r86ImJXUyR4i7kz3Q4JM7b2Xuk/edit?usp=sharing)
- [AI generated quiz for section 4.3 The Slice Ty:Wpe](https://gemini.google.com/share/398908ec9322)

### Example code
- [Chapter 4_1 | Ownership](../4/4_1_What_is_Ownership/ownership/src/main.rs)
- [Chapter 4_1 | Ownership and Functions](../4/4_1_What_is_Ownership/ownership_and_functions/src/main.rs)
- [Chapter 4_1 | Return Values](../4/4_1_What_is_Ownership/return_values/src/main.rs)
- [Chapter 4_2 | References and Borrowing | Calculate length example](../4/4_2_References_and_Borrowing/calculate_length/src/main.rs)
- [Chapter 4_2 | References and Borrowing | Change string example](../4/4_2_References_and_Borrowing/change_string/src/main.rs)
- [Chapter 4_2 | References and Borrowing | Dangling Reference example](../4/4_2_References_and_Borrowing/dangling_references/src/main.rs)
- [Chapter 4_2 | References and Borrowing | Reference Scope example](../4/4_2_References_and_Borrowing/reference_scope/src/main.rs)



