# Lists

Lists are very similar to arrays in other languages. They are a collection of values, and can be accessed by index. And each of the values must be of the same type.

```buri
myList = [1, 2, 3]
```

## Type declarations

```buri
Numbers = [Num]
Strings = [Str]

numbers: Numbers = [1, 2, 3]
strings: Strings = ["a", "b", "c"]
```

## Usage

Items can be accessed by index, and the resulting value is an `Option(t)`:

```buri
numbers = [1, 2, 3]
number = numbers[0]

exists = when number is
    #some(n) => #true
    #none => #false
```

Items can also be destructured:

```buri
numbers = [1, 2, 3]
[n1, n2, n3, n4] = numbers
-- n1 is #some(1), n2 is #some(2), n3 is #some(3), n4 is #none
```
