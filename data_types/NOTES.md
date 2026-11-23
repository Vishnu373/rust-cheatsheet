# Statically typed language
- **Must** know the type of variable at **compile time**.

---

# Types of data
- **Scalar** -> single value => {integers, floating-point numbers, booleans, characters}
- **Compound** -> grouping multiple values => tuples, arrays

---

## Integers
- `i` -> **signed** meaning positive and negative integers
- `u` -> **unsigned** meaning positive integers only
- Available options => {`i8`/`u8`, `i16`/`u16`... `i128`/`u128`, `isize`, `usize`}
- the number beside the `i` or `u` indicates the amount of memory it uses.
    - Equation:-
        - For `i`, `-2^(n - 1)` to `2^(n - 1) - 1`
        - For `u`, `0` to `2^(n) - 1`
        - where `n` is the bit or number beside `i` or `u`.
    - Example:- `i8` means 8 bits / 1 byte of memory is used.
    - `i32` is the **default** for integers.

---

## Floating point
- All are **signed** (positive and negative)
- 2 options -> `f32` and `f64` **(default)**

---

## Boolean type
- 2 types -> `true` or `false`.

---

## char type
- Holds **single** character - alphabets, numbers, special characters and symbols.
- Wrapped in **single quotation marks**.

---

## Operators
- Addition (`+`)
- Subtraction (`-`)
- Multiplication (`*`)
- Division (`/`)
    - Integer division truncates toward zero (e.g. `5 / 3` is `1`, `-5 / 3` is `-1`).
- Remainder or Modulo in other languages (`%`)

---

## tuple
- **Fixed** length; can't be changed later.
- Elements can be of **different** type.
- type annotation is optional for each value.
- Empty tuples are called **unit**.
- Tuples can be mutable by using `mut` keyword.

---

## Array
- All elements must be **same** type.
- **Fixed** length.
- Use it when you know the number of elements in the array will be fixed.
- Allocated in the **stack**.