## Rust by Example

My compiled code examples with completed activities.

### External References

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)

### Table of Contents
- [1_1 Comments](./1_hello_world/1_1_comments/comments/src/main.rs)
- [1_2 Formatted_print](./1_hello_world/1_2_formatted_print/formatted_print/src/main.rs)
- [1_2 Formatted print | Debug](./1_hello_world/1_2_formatted_print/debug/src/main.rs)
- [2_1 Literals and Operators](./2_primitives/2_1_literals_operators/literals/src/main.rs)
- [2_2 Tuples](./2_primitives/2_2_tuples/tuples/src/main.rs)
- [2_3 Arrays and Slices](./2_primitives/2_3_arrays_and_slices/arrays_slices/src/main.rs)
- [2 Primitives | Compound Types](./2_primitives/compound_types/src/main.rs)
- [3_1 Structures](./3_custom_types/3_1_structures/structures/src/main.rs)
- [4 Variable Bindings](./4_variable_bindings/binding/src/main.rs)
- [4_1 Mutability](./4_variable_bindings/4_1_mutability/mutability/src/main.rs)
- [4_2 Scope](./4_variable_bindings/4_2_scope_and_shadowing/scope/src/main.rs)
- [4_2 Shadowing](./4_variable_bindings/4_2_scope_and_shadowing/shadowing/src/main.rs)
- [4_3 Declare_first](./4_variable_bindings/4_3_declare_first/declare_first/src/main.rs)
- [4_4 Freezing](./4_variable_bindings/4_4_freezing/freezing/src/main.rs)
- [5_2 Literals](./5_types/5_2_literals/literals/src/main.rs)
- [7 Expressions](./7_expressions/expressions/src/main.rs)
- [8_1 if/else](./8_flow_of_control/8_1_if_else/if_else/src/main.rs)
- [8_2 loop](./8_flow_of_control/8_2_loop/loop_example/src/main.rs)

### Extras
#### Running cargo clean recursively

```bash
$ cd top_level_of_projects
$ find . -type d -execdir cargo clean \;
```