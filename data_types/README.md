# Data Types

This is a Rust program that demonstrates the different primitive and composite data types.

## Primitive Data Types

### Signed Types

- `i8`: 8-bit
- `i16`: 16-bit
- `i32`: 32-bit
- `i64`: 64-bit
- `i128`: 128-bit
- `isize`: signed integer that depends on the architecture (32 bits on 32-bit architectures and 64 bits on 64-bit
  architectures)

### Unsigned Types

- `u8`: 8-bit
- `u16`: 16-bit
- `u32`: 32-bit
- `u64`: 64-bit
- `u128`: 128-bit
- `usize`: unsigned integer that depends on the architecture (32 bits on 32-bit architectures and 64 bits on 64-bit
  architectures)

### Floating-Point Types

- `f32`: 32-bit floating-point number
- `f64`: 64-bit floating-point number

### Boolean Type

- `bool`: boolean value (true or false)

### Character Type

- `char`: a 4-byte Unicode character

### Composite Data Types

#### Tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1);

A tuple is a group of values with different types. The values are grouped within parentheses and separated by commas.

#### Array

    let a: [i32; 5] = [1, 2, 3, 4, 5];

An array is a collection of values of the same type. Arrays have a fixed length and are defined with square brackets.
