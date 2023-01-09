# Syntax cheatsheet

This page contains the syntax examples for the Buri language. It's mainly used to quickly see all the different syntaxes and ensure they are consistent and quickly parsable. Should the syntax for any feature on another design doc conflict with this doc, this doc takes precedence.

```buri
-- single line comment
-! documentation comment, only spans a single line

-- Type primitives

Str -- string
Int -- integer

-- type identifiers

Numbers = [Num] -- list
Record = {
    name: Str, -- key-value pair
    age?: Nat, -- optional key-value pair
} -- record
Person = {
    name = '', -- default record value
    age = 0, -- default record value
}
Bool = #true | #false -- tag
Option(t) = #some(t) | #none -- t is a type variable
Nested = Option(Bool) -- states: #some(#true) | #some(#false) | #none
PrimaryColors = #red | #green | #blue -- tag group
Colors = PrimaryColors | #yellow | #cyan | #magenta -- union -> #red | #green | #blue | #yellow | #cyan | #magenta
Result(ok, err) = #ok(ok) | #err(err) -- ok and err are type variables

-- Open-ended types
Person(a) = extends({
    name: Str,
    age: Nat,
})
Color = extends(#red | #green | #blue)

-- operators
= -- assignment
+ -- addition
- -- subtraction
* -- multiplication
/ -- division
% -- modulus
++ -- string concatenation
> -- greater than
< -- less than
>= -- greater than or equal to
<= -- less than or equal to
== -- equal to

-- Variable assignment
myVar = 5
myList: [Num] = []

-- lists
myList = [1, 2, 3]
value = myList[0] -- Result(Num, #outOfBounds)
[num1, num2, num3] = myList -- destructuring, each element is a Result(Num, #outOfBounds)

-- tags
emotion = #happy -- emotion is of type #happy
color: Color = #red -- color is of type Color
formattedColor = #rgb(255, 0, 0) -- formattedColor is of type #rgb(Num, Num, Num)

-- records
person = {
    name: "Alice",
    age: 42,
} -- type { name: Str, age: Num }
name = person.name -- Str
age = person.age -- Num
{ name, age } = person -- destructuring, name is a Str, age is a Num
{ myName: name, myAge: age } = person -- destructuring with renaming, myName is a Str, myAge is a Num
{ name = '', age = 0 } = ghost -- set default values if keys are #option
{ myName: name = '', myAge: age = 0 } = person -- with renaming and default values

-- functions
myFunc = (x) => x + 1 -- function with one parameter of type Num
myFunc2 = (x: Num): Num => x + 1 -- function type declarations

-- traits
MyTrait(t) = trait: -- define a trait with variable
    toString = (self) => Str -- define a function in the trait
    setLength = (self, Nat) => self -- accepts a Nat and returns
    getId = (self) => T -- returns a Nat
    Person = {}
extend Person with MyTrait(Num): -- implement the trait for Num
    toString = (_) => "hello world"
    setLength = (self, length) => {...self, length }
    getId = (_) => 4
extend Person with MyTrait1, MyTrait2: -- implement multiple traits
    -- ...
extend Person: -- implement unnamed traits for Person
    -- ...

-- control flow
item = if x == 1 do
    "one"
else if x == 2 do
    "two"
else
    "other"

if condition do doSomething(#true)
color = if #true do #green else #red

toString = (color) =>
    when color is -- pattern matching
        #red do "red"
        #green do "green"
        #blue do "blue"

Character = #person({ age: Int }) | #animal

isOldPerson = (character: Character) =>
    when character is
        #person({ age }) if age > 42 do #true -- with if clause
        _ do #false -- catch all

isPrimary = (color) =>
    when color is
        #red | #green | #blue do #true -- multiple possible values
        #magenta do #false

-- optional chaining
lastName = user?.name?.last -- with properties
array?:push(b) -- with functions
lastName = user?.name?.last ?? "Unknown" -- guards so lastName is always defined

-- module exports
@export
Color = #red | #green | #blue -- export type

-- module imports
import Color, toString, hello from './colors.buri' -- import type, variable, and function
import Color as PrimaryColor from './colors.buri' -- renaming imports
import
    Color as PrimaryColor,
    toString as primaryColorToString,
    hello as helloWorld,
    from './colors.buri' -- multiline renamed imports
```
