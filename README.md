# Rust Cheatsheet

> **Reference** - [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)

- **cargo** - rust package manager; adding dependencies; running the file etc.
- **crate** - packages of code. (Eg:- libraries, standalone program)

---

## Creating a new project/directory
```bash
cargo new project_name
```

## Make a project cargo specific format
```bash
cd project_folder
cargo init
```

## Building a project
Meaning converting the human readable code to machine code.
```bash
cargo build
```

## Build and compile/run the code
Meaning conversion + executing the file.
```bash
cargo run
```

## Check for errors
```bash
cargo check
```

## When the application is ready for release
Creates the release files (like the .exe or installer files etc)
```bash
cargo build --release
```

---

## Lessons
- [1. Variables](variables)
- [2. Data types](data_types)
- [3. Functions - statements and expressions](functions)