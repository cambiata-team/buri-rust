# Control flow

## If / else

If/else statements in Buri can be used like this:

```buri
if conditionA do
    doSomething(#true)
else if conditionB do
    doSomething(#true)
else
    doSomething(#false)
```

Like many other languages, not all parts are needed. You can simply have the `if` block, or just a `if` and `else` blocks, or 20 `else if` blocks in between the two.

Note that in Buri, `else if` is not a special case or keyword. The above example is equivalent to:

```buri
if conditionA do
    doSomething(#true)
else
    if conditionB do
        doSomething(#true)
    else
        doSomething(#false)
```

This is because every part of an if/else statement is an expression.

```buri
if expression do expression else expression
```

Though for readability, it's convention to write the `else if condition do` on a single line.

### Single-line if / else

If statements can be written on a single line:

```buri
if condition do doSomething(#true)
```

```buri
color = if #true do #green else #red
```

### Truthiness

Buri does not have the concept of "truthiness" that other languages have. Truthiness is when an expression that's not true or false is treated as a boolean. As such, the following code will not work:

```buri
list = [1, 2, 3]

-- size(List(a)) => Nat
if list:size() then -- compile error
    #isNotEmpty
else
    #isEmpty
```

Likewise, you cannot use an `if` statement to check if an item exists:

```buri
list = [1, 2, 3]
-- list[0] is type Option(Num)
if list[0]
    -- ...
```

Instead, all expressions used in `if` statements must evaluate to `#true` or `#false`.

```buri
list = [1, 2, 3]
if list:size() > 0 then
    #isNotEmpty
else
    #isEmpty
```

### Missing else

Sometimes you may want an if statement without the else clause. In those instances, the `if` block returns an `Option`:

```buri
color: Option(#green) = if #true then #green
```

## Pattern matching

Pattern matching is an extremely powerful feature. It allows you to match a value against several patterns and execute code based on the matched pattern.

```buri
toString = (color) =>
    when color is
        #red do "red"
        #green do "green"
        #blue do "blue"
```

In this example, `color` has the type of `#red | #green | #blue`. The `match` statement compares it's value to each of the tags and returns the corresponding string.

```buri
red = toString(#red) -- "red"
green = toString(#green) -- "green"
blue = toString(#blue) -- "blue"
```

Pattern matching doesn't simply work on tags. It works on any data structure:

> **Note:** More thinking should be used if/how pattern matching works with functions, maps, and sets.

```buri
toString = (num) =>
    when num is
        1 do "one"
        2 do "two"
        -- ...
```

```buri
toNum = (str) =>
    when str is
        "one" do 1
        "two" do 2
        -- ...
```

### Tag values

`match` statements can also match against tag values.

```buri
IpAddr =
    | #ipAddrV4(U8, U8, U8, U8)
    | #ipAddrV6(Str)

isLocalHost = (ip: IpAddr) =>
    when ip is
        IpAddrV4(127, 0, 0, 1) do #true
        IpAddrV6("::1") do #true
        _ do #false
```

Here, the first clause only runs if `ip` is an `#ipAddrV4` with the values `127, 0, 0, 1`. The second clause only runs if `ip` is an `#ipAddrV6` with the value `::1`. The `_` is a catch-all that matches all other options.

Alternatively, you can bind new variables to the tag values:

```buri
IpAddr =
    | #ipAddrV4(U8, U8, U8, U8)
    | #ipAddrV6(Str)

parseIpAddr = (ip: IpAddr) =>
    when ip is
        IpAddrV4(a, b, c, d) => "IPv4 with values: ${a}, ${b}, ${c}, ${d}"
        IpAddrV6(str) => "IPv6 with address: ${str}"

ipString = parseIpAddr(#ipAddrV4(127, 0, 0, 1)) -- "IPv4 with values: 127, 0, 0, 1"
ipString2 = parseIpAddr(#ipAddrV6("::1")) -- "IPv6 with address: ::1"
```

### Multi-line clauses

Clauses can be multiple lines. Just use a code block:

```buri
toString = (color) =>
    when color is
        #red do
            red = "r" ++ "e" ++ "d"
            red
        #green do "green"
        #blue do "blue"
```

### If conditionals

Sometimes you may wish to use if statements while pattern matching. That is allowed:

```buri
Character = #person({ age: Int }) | #animal

isOldPerson = (character: Character) =>
    when character is
        #person({ age }) if age > 42 do #true
        _ do #false
```

### Exhaustive matching

Pattern matching must be exhaustive—it must match all possible patterns. In fact, aside from the cleaner syntax, the exhaustive checks are one of the primary reasons why pattern matching is preferred over classic if-else chains.

The following will produce a compile error since not all branches are covered. Notice how the `match` statement doesn't account for when `color = #blue`.

```buri
type Color = #red | #green | #blue

toString = (color: Color) =>
    when color is
        #red do "red"
        #green do "green"
        -- missing #blue
```

### Catch-all

Sometimes you may wish to only take action for a few values, but the rest you'd like to ignore (or do the default behavior). For that, you can use an `_` as a catch-all:

```buri
toString = (num) =>
    when num is
        1 => "one"
        2 => "two"
        _ => "other"

one = toString(1) -- "one"
two = toString(2) -- "two"
other = toString(3) -- "other"
```

Note that the catch-all must be the last clause in the match statement because each statement is matched in order. If it's not last, all patterns after the catch-all will be ignored:

```buri
toString = (num) =>
    when num is
        1 do "one"
        _ do "other"
        2 do "two" -- can never run

one = toString(1) -- "one"
two = toString(2) -- "other"
other = toString(3) -- "other"
```

If you do this, your code will still compile but the compiler will raise a warning.

You can also use a catch-all inside of destructuring or pattern matching on tag values:

```buri
Character = #person(age: Nat) | #animal

isPerson = (character: Character) =>
    when character is
        #person(_) do #true
        _ => #false
```

There are some downsides to catch-all patterns:

- No exhaustive checks are performed. Because the catch-all matches every single pattern, if the input's type definition expands (e.g., from `#red | #green | #blue` to `#red | #green | #blue | #purple`), the compiler will not warn you that you're missing a pattern as all patterns are still covered—even if you wanted to something different with `#purple`.
- You'll get weaker type inference. For instance, if you have:

    ```buri
    when color is
        #red do "red"
        #green do "green"
        #blue do "blue"
    ```

    the compiler will infer that `color` is `#red | #green | #blue`. However, if you include a catch-all, color could be any tag:

    ```buri
    when color is
        #red do "red"
        #green do "green"
        #blue do "blue"
        _ => "other"
    ```

    The compiler will infer that `color` is `extends(#red | #green | #blue)`
