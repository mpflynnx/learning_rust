## Rust by Example

### External References

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)

### Table of Contents
- [1_1_comments/comments](./1_hello_world/1_1_comments/comments/src/main.rs)
- [1_2_formatted_print/debug](./1_hello_world/1_2_formatted_print/debug/src/main.rs)
- [1_2_formatted_print](./1_hello_world/1_2_formatted_print/formatted_print/src/main.rs)
- [2_1_literals_operators](./2_primitives/2_1_literals_operators/literals/src/main.rs)
- [2_2_tuples/tuples](./2_primitives/2_2_tuples/tuples/src/main.rs)
- [2_3_arrays_and_slices/arrays_slices](./2_primitives/2_3_arrays_and_slices/arrays_slices/src/main.rs)
- [2_primitives/compound_types](./2_primitives/compound_types/src/main.rs)
- [4_variable_bindings](./4_variable_bindings/binding/src/main.rs)
- [4_1_mutability](./4_variable_bindings/4_1_mutability/mutability/src/main.rs)
- [4_2_scope_and_shadowing/scope](./4_variable_bindings/4_2_scope_and_shadowing/scope/src/main.rs)
- [4_2_scope_and_shadowing/shadowing](./4_variable_bindings/4_2_scope_and_shadowing/shadowing/src/main.rs)
- [4_3_declare_first](./4_variable_bindings/4_3_declare_first/declare_first/src/main.rs)
- [4_4_freezing](./4_variable_bindings/4_4_freezing/freezing/src/main.rs)
- [5_2_literals](./5_types/5_2_literals/literals/src/main.rs)
- [7_expressions/expressions](./7_expressions/expressions/src/main.rs)
- [8_1_if_else](./8_flow_of_control/8_1_if_else/if_else/src/main.rs)
- [8_2_loop](./8_flow_of_control/8_2_loop/loop_example/src/main.rs
)

### Extras
#### Running cargo clean recursively

```bash
$ cd top_level_of_projects
$ find . -type d -execdir cargo clean \;
```