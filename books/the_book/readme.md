## The Rust Book

### External References
- [The Rust Programming Language](https://doc.rust-lang.org/stable/book/title-page.html)

### My code and documentation
- [Chapter 1 | hello_world](1/chapter1_readme.md)
- [Chapter 2 | guessing-game](2/chapter2_readme.md)
- [Chapter 3 | Common Programming Concepts](3/chapter3_readme.md)
- [Chapter 4 | Understanding Ownership](4/chapter4_readme.md)
- [Chapter 11 | Writing Automated Tests](11/chapter11_readme.md)

### Extras

#### Prevent `cargo new` creating a .git folder

- Linux user configuration for [config.toml](https://doc.rust-lang.org/cargo/reference/config.html)

```bash
$ cd ~
$ touch .cargo/config.toml
```
- Add line to `.cargo/config.toml`
```toml
[cargo-new]
vcs = "none" 
```

#### Running cargo clean recursively

```bash
$ cd top_level_of_projects
$ find . -type d -execdir cargo clean \;
```
