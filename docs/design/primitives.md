# Primitives

Unlike many languages, Buri only has 2 primitive data types: Strs and numbers.

## Strings

Items with the `Str` type are delimited by either single quotes or double quotes. All strings are UTF-8.

```buri
hello = 'hello, world!'
hi = "hi, world!"
```

Use `Str` to annotate a string type:

```buri
hello: Str = 'hello, world!'
```

### String interpolation

```buri
name = 'Bob'
greeting = 'Hello ${name}!' -- 'Hello Bob!'
```

Interpolated segments are start with `${` and end with `}`. Any expression can go inside there as long as it evaluates to a string.

### Concatenation

String concatenation uses `++`:

```buri
name = 'World'
greeting = 'Hello, ' ++ name ++ '!'
```

### Escape sequences

In both strings and interpolated strings, you can use escape sequences to insert characters into the string that would otherwise not be allowed.

| Sequence | Character       |
| -------- | --------------- |
| `\'`     | single quote    |
| `\"`     | double quote    |
| `\n`     | newline         |
| `\t`     | tab             |
| `\r`     | carriage return |
| `\\`     | backslash       |
| `\$`     | dollar sign     |

If a backslash precedes any other character, it is ignored. For instance, `'\w'` is equivalent to `'w'`.

## Numbers

Numbers can be divided into two categories: `Float`s and `Int`s.

```buri
isAnInteger = 2 -- see below for exact details
isAFloat = 2.0
```

### Sizes

Buri supports two types of floats:

- `F32`: 32-bit floating point (IEEE 754)
- `F64`: 64-bit floating point (IEEE 754)

However, Buri does not allow operations to evaluate to `Infinity`, `-Infinity`, or `NaN`. Operations like `sqrt` or `/` that may evaluate to these will return a [Result](tags.md#result).

Like floats, Buri also supports multiple types of integers:

| Length | Signed | Unsigned |
| ------ | ------ | -------- |
| 8-bit  | `I8`   | `U8`     |
| 16-bit | `I16`  | `U16`    |
| 32-bit | `I32`  | `U32`    |
| 54-bit | `I54`  | `U54`    |
| 64-bit | `I64`  | `U64`    |
| 128-bit| `I128` | `U128`   |

> **Note:** 54-bit integers should never be explicitly used (you will get compiler warnings if you use it). They are only used for platforms without native integer support (e.g., JavaScript) and therefore require emulation through floats.

Furthermore, Buri supports four more general keywords:

- `Num`: a number of any type
- `Int`: an integer of any size
- `Nat`: an unsigned integer (i.e., natural number) of any size
- `Float`: a floating point number of any size

### Number type identifiers

Like Roc, Buri actually defines these variants of numbers as type identifiers. For instance:

- `I64` is a type identifier for `Num (Integer (Signed S64))`
- `U8` is a type identifier for `Num (Integer (Unsigned U8))`
- `F32` is a type identifier for `Num (FloatingPoint F32)`
- `Num` is a type identifier for `Num *`
- `Int` is a type identifier for `Num (Integer *)`
- `Nat` is a type identifier for `Num (Integer (Unsigned *))`
- `Float` is a type identifier for `Num (FloatingPoint *)`

> **Note:** While these underlying types exist (e.g., `Num (Integer (Signed S64))`), you cannot use them directly. You must use the type identifiers. The only way you can use these underlying types is with [generics](type-identifiers.md#generics).

This allows you to create more generalized number types, useful for things like functions that can operate on any type of number. For instance, `(Num<a>, Num<a>) => Num<a>` is a function type that accepts any two numbers and returns a number, but all the numbers must be of the same type. Likewise, `(Num<*>, Num<*>) => Int` is a function type that accepts any two numbers (even of different types) and returns an integer of any type.

### Type inference

Type inference for numbers in Buri works very similarly to type inference for numbers in Roc.

By default, all numbers are inferred to the most general type. Number literals without a decimal point are inferred as `Num<*>`, and those with a decimal point are inferred as `Float<*>`.

Most of the time, these inferences can be ruled down into something more specific. For instance, if you have a function that accepts integers of a specific size, the type will be inferred with that size:

```buri
one = 1 -- is a U8

parseByte = (b: U8) => b
parseByte(one)
```

Likewise, if you have a function that accepts a `Float`, a `Num *` will be narrowed down to `Float`:

```buri
one = 1 -- is a Float

parseFloat = (b: Float) => b
parseFloat(one)
```

If for some reason the context does not narrow down the type, `Num<*>` will be treated as an `Int<*>`. The exact size will depend on the platform. For instance, on a modern 64-bit CPU, `Num<*>` and `Int<*>` will be treated as `I64`, and `Float<*>` will be treated as `F64`. Alternatively, when compiling to JavaScript (which doesn't have a native integer type), `Num<*>` and `Int<*>` will be a `I54` while `Float<*>` will be a `F64`.

### Type conversion

In Buri, a more general number type can always be converted into a more specific type. For instance:

- `Num *` can be converted to `Int *` or `Float *`
- `Int *` can be converted into `Num (Integer (Unsigned *))`

This means you can do things like this:

```buri
one: Int = 1
two: Float = 2.0
three = one + two -- is a Float
```

> **Note:** Type conversion between numbers and any other type is not supported. For instance, `string = "hello" ++ 3` produces a compile error.

### Formatting

Like JavaScript, numbers can have underscores `_` for readability:

```buri
largeNumber = 1_234_567
largeFloat = 1_234.567_89
```

Underscores can appear anywhere in the number (aside from the start or end of the digit sequence), not just every third digit.

Additionally, for floats, scientific notation can be used:

```buri
thousand = 1e3
isThousand = thousand == 1000.0 -- #true
```

Buri also supports hexadecimal, octal, and binary integer literals:

```buri
hexadecimal = 0x01
octal = 0o01
binary = 0b01
```

Each of these hexadecimal, octal, and binary literals are inferred as `Int`, not `Num`.

### Integer overflow and underflow

Assume you have a variable `myNum: U8`. A `U8` can only hold numbers from 0–255 inclusive. So if you write this…

```buri
myNum: U8 = 255
addedNum = myNum + 1 -- overflow!
```

…you get an overflow! Likewise, if you do this…

```buri
myNum: U8 = 0
subtractedNum = myNum - 1 -- underflow!
```

…you get an underflow!

Buri handles these cases interestingly (though in a very similar way to Rust). In development, Buri will include runtime checks on all operations that may result in an integer overflow or underflow. If an overflow/underflow occurs, the program will terminate with an error. That way you know to fix the issue before you release your code.

However, for production builds, Buri will not perform runtime checks to improve performance. That means overflows and underflows may occur in production. The exact behavior depends on the platform. For compiled binaries, Buri will perform two's compliment wrapping. For compiling to another language, the behavior will entirely depend on how that language handles overflows and underflows. As such, relying on integer overflow or underflow is considered an error and should be avoided.

To explicitly handle overflow or underflow, the standard library has a variety of functions to replace common operations.

> Aaron note: use two's compliment in memory.
