# Records

Records are very similar to structs in other languages or objects in JavaScript. They have fixed fields.

## Creation

You can declare a type like this:

```buri
Person = {
    name: Str,
    age: Nat,
}
```

Then, you can use the type to create a new record:

```buri
me: Person = {
    name: "John",
    age: 42,
}
```

Note that the record must include all the fields of the type (except optional values, see below).

### Optional fields

Some fields in a record can be optional. To make a field optional, just use a `?` in the definition. In this example, `age` is optional:

```buri
Person = {
    name: Str,
    age?: Nat,
}

me: Person = {
    name: "John",
}

you: Person = {
    name: "Jane",
    age: 42,
}

# both myAge and yourAge have type Option(Nat)
myAge = me.age
yourAge = you.age
```

Note, optional fields are distinctly different than fields that are options. For instance, this will not compile:

```buri
Person = {
    name: Str,
    age: Option(Nat),
}

-- compile error, age is required
me: Person = {
    name: "John",
}
```

When using a field that's an option, you must still define it:

```buri
Person = {
    name: Str,
    age: Option(Nat),
}

me: Person = {
    name: "John",
    age: None,
}
```

### Shorthand

If you're defining a record where a variable's name is the same as the field name, you can omit the field value:

```buri
Person = {
    name: Str,
    age: Nat,
}

name = "John"
me: Person = {
    name, -- instead of `name: name`
    age: 22,
}
```

## Usage

You can access a record's fields with the dot operator:

```buri
me.name -- "John"
```

### Updates

You can update a record's fields with the spread operator:

```buri
olderMe: Person = {
    ...me,
    age: me.age + 1,
    name: "Older " + me.name,
}
```

Note that you cannot use the spread operator to add new fields.

Alternatively, if you wish to update just one field, all records have a `set` [trait](traits.md) for each of their fields.

```buri
olderMe: Person = me:setAge(me.age + 1)
```
