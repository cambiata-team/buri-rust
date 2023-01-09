# Type system

All variables have a type. And this type obeys certain properties:

- Static: A variable is always assigned with a type, and it will never change types.
- Sound: The type will never be wrong. This allows us to eliminate all runtime type checks.
- Inferred: The language has complete type inference, so type annotations are not necessary.

This provides you the safety and guarantees normally associated with low-level code with the flexibility and ease of use of high-level code.

See also:

- [Type identifiers](type-identifiers.md)
- [Type inference](type-inference.md)
