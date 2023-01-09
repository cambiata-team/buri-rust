# Type identifiers

Type identifiers always begin with an uppercase letter. You can then assign a type expression to a type identifier. For instance, here's a type declaration for a record:

```buri
Person = {
    name: Str,
    age: Option(Nat),
}
```

There are three parts to a type definition:

1. The name of the identifier
2. The `=` sign
3. A valid type expression (could be another identifier)

Type identifiers can be assigned any type expression.

```buri
PrimaryColor = #red | #green | #blue
SecondaryColor = #yellow | #orange | #purple
Rainbow = PrimaryColor | SecondaryColor -- equivalent to #red | #orange | #yellow | #green | #blue | #purple
Point = { x: Float, y: Float }
Names = List(Str)
VisitedPeople = Set(Str)
```

## Usage

Once a type identifier is defined, you can use it as you would any other type. Here, we've defined a type of `Person`, and created a variable `john` of type `Person`:

```buri
Person = {
    name: Str,
    age?: Nat,
}

john: Person = {
    name: "John",
}
```

## Nominal vs. structural equality

Many languages have a distinction between [nominal and structural equality](https://medium.com/@thejameskyle/type-systems-structural-vs-nominal-typing-explained-56511dd969f4). Buri does not have nominal equality. As long as the structure of two types match, they are considered identical. This matches the conventions of many functional languages and TypeScript.

```buri
Student = {
    name: Str,
    age: Nat,
}

Teacher = {
    name: Str,
    age: Nat,
}

personA: Teacher = {
    name: "John",
    age: 42,
}

personB: Student = personA -- it works
```

## Generics

Let's say we wanted to write a function that adds two numbers:

```buri
add = (a, b) => a + b
```

What should the type annotation be? You might say it would be `(Num, Num) => Num`, but that means we might pass in a `U8` and `F32`. But really we want to pass in two numbers of the same type. In reality, we want a generic.

Generics are defined with `(` and `)`. So the type annotation would be:

```buri
-- TODO: figure out the syntax for generics in this use case
add: (Num(a), Num(a)) => Num(a) = (a, b) => a + b
```

This says that we accept two numbers of the same type, then return a number of that same type.

Or consider generics with tag groups:

```buri
Result(ok, err) = #ok(ok) | #err(err)
```

This says that result is an identifier for `#ok` or `#err`. But both `#ok` and `#err` can have values of any type. Note that generics always start with a lower case letter, while all other types start with an uppercase letter.

However, if we have a function of type:

```buri
(List(Num)) => Result(Num, #outOfBounds)
```

The return type is equivalent to:

```buri
#ok(Num) | #err(#outOfBounds)
```

## Combining tag groups

Tag groups can be combined. Take a look at the following example:

```buri
PrimaryColor = #red | #green | #blue
SecondaryColor = #yellow | #orange | #purple
Rainbow = PrimaryColor | SecondaryColor -- equivalent to #red | #orange | #yellow | #green | #blue | #purple
```
