# Statically typed language
- Must know the type of variable at compile time.

---

# Types of data
- Scalar -> single value => {integers, floating-point numbers, booleans, characters}
- Compound

---

## Integers
- i -> signed meaning positive and negative integers
- u -> unsigned meaning positive integers only
- Available options => {i8/u8, i16/u16... i128/u128, isize, usize}
- the number beside the i or u indicates the amount of memory it uses.
    - Equation:-
        - For i, -2^(n - 1) to 2^(n - 1) - 1 
        - For u, 0 to 2^(n) - 1
        - where n is the bit or number beside i or u.
    - Example:- i8 means 8 bits / 1 byte of memory is used.
    - i32 is the default for integers.

---