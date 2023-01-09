# Tags

Tags allow you to a type which may be one of a few different values. They behave similarly to enums in Rust and Swift, but with a more user-friendly syntax.

## Declaration

Tags always start with an uppercase letter, and they are used directly.

```buri
green = #green
```

Here, `green` has a value of the `#green` tag.

You can also create a type which is a collection of tags:

```buri
Color = #red | #green | #blue

green: Color = #green
```

### Tags with data

Tags can also contain data:

```buri
localhost = #ipAddrV4(127, 0, 0, 1)
```

Now the variable `localhost` has a value of the `#ipAddrV4` tag, which contains the `Num`s `127`, `0`, `0`, and `1`. You'll often use these values with pattern matching.

And the data can be incredibly useful when using types for tags groups:

```buri
IpAddr =
    | #ipAddrV4(U8, U8, U8, U8)
    | #ipAddrV6(Str)

localhost: IpAddr = #ipAddrV4(127, 0, 0, 1)

parseIpAddress = (ip: IpAddr) =>
    when ip is
        #ipAddrV4(a, b, c, d) => parseIpV4(a, b, c, d)
        #ipAddrV6(s) => parseIpV6(s)

parsedLocalhost = parseIpAddress(localhost)
```

> **Note:** in this example, annotating the `IpAddr` type for both `localhost` and the function argument are unnecessary, but they are here to more clearly demonstrate how tag data can be used with a defined type.

Sometimes simply adding types into a tag may be confusing as you don't know what the data represents.

```buri
Character = #person(Nat)
```

## Global enums

There are a few enums in the global scope by default due to their prevalent use in Buri programs: Booleans, Options, and Results.

### Booleans

In Buri, booleans are actually tags, not a unique primitive. In short, `Bool` is itself actually a tag group:

```buri
Bool = #true | #false
```

### Options

Buri does not have any concept of null or undefined. However, sometimes it is useful to represent the absence of a value. That's where options come in.

Here's the definition:

```buri
Option(t) = #some(t) | #none
```

You can use it like this:

```buri
isSomething = #some(5)
isNothing = #none

-- notice how the entire type definition of this function
-- is inferred from the single 0. Cool, right?
getNumber = (num) =>
    when num is
        #some(n) => n
        #none => 0

five = getNumber(isSomething)
zero = getNumber(isNothing)
```

Unlike other languages like Rust, Options actually occur infrequently in Buri. That's because most of the time, a result is the better choice.

### Results

Results are a great way to represent the success or failure of an operation. Unlike throwing errors, using a Result enum allows us to explicitly handle errors producing more robust and secure applications. Here's the definition:

```buri
Result(ok, err) = #ok(ok) | #err(err)
```

Most of the time, the error type `err` will also be another tag. This allows for very detailed error handling.

For instance, let's create a function that accepts a list of numbers and an index, and returns the number at that index. If the index is out of bounds, it returns 0.

```buri
getNthNumber = (list: List(Num), n: Nat): Num =>
    item = list[n] -- type is Result(Num, #outOfBounds)
    when item is
        #ok(value) => value
        #err(#outOfBounds) => 0
```

> **Note:** like many examples in the design, the type annotations are not necessary here, but are used to make it easier to understand what's going on.
