# Variable declaration

All variables are declared with equals sign keyword.

```buri
a = 5
meaningOfLife = 42
string = "hello world"
```

All variables are immutable (with the exception of [internal reassignment](internal-reassignment.md)), and must start with a lowercase letter (to eliminate confusion with types).

> **Note:** In JavaScript, you have multiple ways to declare variables: `var`, `let`, and `const`. In Buri, there's only one.

## Type inference and declaration

By default, when you declare a variable, the type will be inferred. It doesn't matter if you use a primitive, function, or something else.

```buri
a = 5 -- inferred as type Num
b = "hello" -- inferred as type Str
```

However, sometimes you may wish to specify the type yourself (e.g., an empty array). In this case, you can specify the type with a colon:

```buri
nums: [Int] = [] -- type is [Int]
a: U8 = 5 -- type is U8 when it would otherwise be inferred as Num
```
