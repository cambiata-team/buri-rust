# Glossary

## Item

A variable, constant, literal, or expression.

## Property

If an item “has property x” it means that a specific action x can be applied to the item. The following is a non-exhaustive list of properties:

- Given the statement `a = 2`, `a` has the property that it can be assigned from the literal `2`.
- Given the statement `y = f(x)`, `x`, `f`, and `y` have the following properties:
  - `x` has the property that it can be passed to `f` as the first argument
  - `f` has the property that it is a function that can be called with `x` as the first argument
  - `y` has the property that it can be assigned from the return value of `f(x)`.
- Given the statement `p + q`, `p` and `q` have the following properties:
  - `p` has the property that it can be added with `q`
  - `q` has the property that it can be added with `p`
- Given the statement `hello.world`, `hello` has the property that `world` is a member of `hello`.
- Given the statement `b[n]`, `b` and `n` have the following properties:
  - `b` has the property that it is indexable by `n`
  - `n` has the property that it can serve as an index for `b`

## Abstract type

An abstract type is a set of properties.

An item satisfies abstract type if and only if the set of properties of the item is a subset of the abstract type.

## Abstract subtype

For abstract types X and Y, X is an abstract subtype of Y if and only if Y is a strict subset of X.

## Concrete type

A concrete type is composed of an abstract type, and an implementation for each of the specific actions of the abstract type’s properties.

A concrete type implements an abstract type if and only if the abstract type is a subset of the concrete type’s abstract type.

An item satisfies a concrete type if the item satisfies the concrete type’s abstract type.

## Concrete subtype

For concrete types X and Y, X is a concrete subtype of Y if and only if the abstract type of X is an abstract subtype of the abstract type of Y.

## Type

An abstract type or a concrete type.

## Trait method

A trait method is a type of property.

Items with this property have a method of a specific name, which accepts arguments of specific types and has return type given only by the type of the variable, constant, or literal, and the types of the method arguments.

## Trait

A set of trait methods.

## Minimum implementation

A specific concrete type is a minimum implementation of a specific abstract type if and only if the following two conditions are met:

- The concrete type implements the abstract type.
- Of all the concrete types in the system of which the specific concrete type is a concrete subtype, none implement the specific abstract type.

## Tag union

An algebraic union of tags, previously also known as a tag group.
