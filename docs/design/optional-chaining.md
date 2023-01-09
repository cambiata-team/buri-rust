# Optional chaining

Sometimes it may be useful to access properties of an object that may or may not exist. This can be achieved by using the `?` operator. This will always result in an [option](tags.md#Options). For instance:

```buri
lastName = user?.name?.last
-- lastName is of type Option(Str)
```

This can also be achieved with chained functions:

```buri
array?:push(b)
```

Note that the expressions resulting type will always be an Option with the type of the last element.

This can be incredibly useful by eliminating many nested pattern matches.

## Guards

You can also use guards to supply a default value when using optional chaining.

```buri
lastName = user?.name?.last ?? "Unknown"
-- lastName is of type Str
```

Guards use the `??` syntax and is similar to nullish coalescing in JavaScript. Essentially, if the value to the left of the `??` evaluates to `#none`, then the value to the right of the `??` will be used. This provides a very easy way to unwrap an optional value.

It is essentially syntactic sugar for the following function:

```buri
guard = (t)(value: Option(t), defaultValue: t) =>
    when value is
        #some(value) do value
        #none do defaultValue
```

As such, the default value must be of the same type as the unwrapped value on the left. You cannot use the default value to union with the unwrapped value. The following will cause a compile error:

```buri
-- user?.name?.last is of type Option(Str)
lastName = user?.name?.last ?? #unknown -- compile error, #unknown is not a Str

-- color is of type Option(#red | #green | #blue)
usedColor = color ?? #yellow -- compile error, #yellow is not of type #red | #green | #blue
```

Technically, though, you can use guards for any optional value, whether or not you're using optional chaining.

```buri
-- isOptional is of type Option(Str)
lastName = isOptional ?? "Unknown"
-- lastName is of type Str
```
