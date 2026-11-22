# Variable
- Think of it as a container to store value. Variables can **mutable** and **immutable**.
- By default variables are **immutable**.
- Limited scope (inside a function).

## Mutable
- Value **can** be changed after assigning a value.

## Immutable
- Value **can't** be changed after assigning a value.

---

# Constants
- Variables that aren't supposed to change and can't be changed upon assigning (in Rust).
- **Must** declare the type.
- Naming convention is use capital letters with underscore between words.
- **Global** scope.

---

# Shadowing
- Declare a new variable with same name again as the previous variable.
- Thus, the compiler will only see the last assigned value when used.
- It is different from mutablity.
- By shadowing, you can change the type of variable upon reassigning it a new value.
    - But if you try to use the `mut` keyword for this purpose, you will get an error.