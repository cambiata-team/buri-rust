# Internal reassignment

> Note: this is not planned for the MVP

Everything in Buri is immutable with one exception: internal reassignment.

If a variable meets the following conditions, it can be reassigned:

1. The variable is declared inside a function (this can include the function's arguments)
2. The reassignment occurs inside the function in which the variable is declared
3. New variables new value has the same type as the previous value
4. There are no outstanding references to the variable

Essentially, a variable internal to a function can be reassigned, hence the name "internal reassignment".

## Reassignment of variables

To reassign a variable, just use `=` again:

```buri
function = (name) =>
    hello = "hello"
    hello = hello + " " + name -- this compiles!
    return hello
```

This allows for many procedural programming patterns to be used:

```buri
getMailtoUrl = (email, options) =>
    mailto =  "mailto:" + email
    if options.subject:isSome() then
        mailto = mailto + "?subject=" + options.subject:get()
    if options.body:isSome() then
        mailto = mailto + "&body=" + options.body:get()
    return mailto
```

Note that reassignment can only occur inside a function's scope. If you try to reassign a variable outside of a function, you will get an error.

```buri
a = 1
a = 2 -- error, reassignment can only occur inside a function
```

Likewise, if you try to reassign a variable that was defined outside of the current function, you will get an error:

```buri
myName = "world"
sayHello = () =>
    myName = "Alice" -- error, reassigning a non-local variable
    "hello " ++ myName

outer = () =>
    x = 1
    inner = () =>
        x = x + 1 -- error, reassigning a non-local variable
        return x
    return inner
```

Finally, if you try to reassign a variable that's still used by an inner function's definition, you will get a compile error:

```buri
outer = () =>
    x = 1
    inner = () => x + 1
    x = 2 -- error, reassigning a "live" variable
    return inner
```

## Reassignment of arguments

Arguments can be reassigned:

```buri

```

## Reassignment is not mutability

It's important to note that just because a variable can be reassigned does not mean it's actually mutable. For instance, the following two examples will not compile because the list and records are not mutable:

```buri
editNumbers = (numbers) =>
    numbers[0] = 100 -- error, variables are not mutable
    return numbers

numbers = [1, 2, 3]
newNumbers = editNumbers(numbers)

editPerson = (person) =>
    person.name = "Bob" -- error, variables are not mutable
    return person

person = {
    name: "Alice",
    age: 42,
}
newPerson = editPerson(person)
```

Instead, use the built-in immutable setters and internal reassignment:

```buri
editNumbers = (numbers) =>
    numbers = numbers:set(0, 100) -- this compiles!
    return numbers

numbers = [1, 2, 3]
newNumbers = editNumbers(numbers)

editPerson = (person) =>
    person = person:setName("Bob") -- this compiles!
    return person

person = {
    name: "Alice",
    age: 42,
}
newPerson = editPerson(person)
```

## Advantages

Internal reassignment allows for a few advantages over fully functional languages:

1. Easier to adopt for developers with a non-functional background
2. Allows for loops (e.g., `for` and `while`)
3. Makes many procedural programming techniques more intuitive to code
4. Allows for more performance optimizations (e.g., while loops eliminate recursive function calls)
5. Allows for using performance optimizations more often (e.g., the compiler doesn't need to guess when a variable's memory can be reused as much)
6. Allows for a faster compiler (e.g., the less optimization a compiler needs to perform, the faster it compiles)
