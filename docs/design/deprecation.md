# Deprecation (WIP)

> This is a work in progress. It may be updated as documentation comments are solidified.

In large code bases, it's common to deprecate code to make sure it's no longer used. However, in most languages this can be a difficult process. In this language, you can easily deprecate functions, variables, and types with the `@deprecate` decorator.

```buri
@deprecate
hello = () => "hello world"
```

Simply by including `@deprecate` before the item, the compiler will give a warning for every instance of the deprecated item. That way, all users of the item will know to update their code, and once all the warnings are gone, you can safely remove the deprecated item.

However, sometimes you need to convey more information about the deprecation. For example, if users should migrate to a new function, you may wish to indicate that.

Luckily, that's really easy. Just add your description as a string inside of parentheses.

```buri
@deprecate('the hello function will be renamed to helloWorld in the next major version')
hello = () => "hello world"
```
