# Functions

## Declaration

All functions are defined using the following syntax:

```buri
(arguments) => expression
```

## Type definitions

Type definitions for functions follow this format:

```buri
(argumentTypes) => returnType
```

For instance, the function `(a, b) => a + b` has the type:

```buri
(Num(a), Num(a)) => Num(a)
```

You can still inline type definitions into the function definition:

```buri
add = (a: Num(a), b: Num(a)) => a + b
```
