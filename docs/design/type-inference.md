# Type inference

Types in Buri are inferred to be the most general type possible. And Buri has complete type inference so you'll never need to annotate types unless you want to.

For instance, look at this function:

```buri
colorToString = (color) =>
    when color is
        Red do "red"
        Green do "green"
        Blue do "blue"

# colorToString is inferred to be of type (#red | #green | #blue) => Str
```

Notice how there are no type annotations, but Buri knows that the input can only be the tags `Red`, `Green`, or `Blue`. That's because `match` statements are exhaustive, so the only valid tags can be those three.
