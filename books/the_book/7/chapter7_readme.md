## Chapter 7. Managing Growing Projects with Packages, Crates, and Modules


### External References

- [Book Chapter 7: Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Modules Cheat Sheet](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet)


### Example code
- [Chapter 7_2 | restaurant example](./7_2_defining_modules_to_control_scope_privacy/restaurant/src/lib.rs)
- [Chapter 7_2 | use keyword examples](./7_2_defining_modules_to_control_scope_privacy/use_example/src/main.rs)
- [Chapter 7_5 | separating modules](./7_5_separating_modules/restaurant/src/lib.rs)

### Creating an library
- Create a new library named `restaurant`.
```bash
$ cargo new restaurant --lib
$ cargo build
```

### 7.5 restaurant example file/directory hierarchy 

```bash
.
└── restaurant
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        ├── back_of_house.rs
        ├── front_of_house
        │   └── hosting.rs
        ├── front_of_house.rs
        └── lib.rs

```
