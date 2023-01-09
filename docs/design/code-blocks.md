# Code blocks

In Buri, code blocks are defined by indentation.

## Default return

All code blocks are expressions. As such, they "return" the value of the last expression.

```buri
five =
    two = 2
    three = 3
    two + three -- this value is "returned"
```

This "return" applies to all code blocks, including if-else statements.

```buri
getLightnessTag = (lightness) =>
    if lightness > 0.5 then
        Light
    else
        Dark

isLight = getLightnessTag(1)
isDark = getLightnessTag(0)
```

Variable assignments always return `None`.

```buri
isNone =
    isOne = 1
```

## Scope

Variables are scoped to code blocks in which they are defined. So every code block defines a new scope.

```buri
a = 5
    -- a is accessible here
    b = 6
    c = a + b
-- a is accessible here
-- b and c are not accessible here
```

Note that this also applies with `if` statements. `if` statements don't use some special type of "if scope", but rather they use the exact same type of scope:

```buri
if true then
    a = 5
-- a is not accessible here
```
