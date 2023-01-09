# Traits

Traits are interfaces that records, lists, maps, sets, or platforms can implement, and are very similar to Rust's traits (with a few notable differences). The following are a few examples of traits:

```buri
Format = trait:
    toString = (self) => Str

Option = trait:
    isSome = (self) => Bool
    isNone = (self) => Bool

Iterable(t) = trait:
    next = (self) => Option(t)
    hasNext = (self) => Bool

Pushable(t) = trait:
    push = (self, t) => self
```

Essentially, a trait is a set of functions that are attached to a type.

## Usage

Then use `extend` to implement a trait:

```buri
Option(t) = #some(t) | #none

extend Option with Option:
    isSome = (self) =>
        return when self is
            #some(_) => #true
            #none => #false

    isNone = (self) =>
        return when self is
            #some(_) => #false
            #none => #true

myOption: Option = #some(1)
myOption:isSome() -- #true
myOption:isNone() -- #false
```

Notice how the first argument of each implemented function is `self`? That's because the item's value is always passed in as the first argument, no matter what the definition of the function is. And you should not add "self" to the trait definition. Like other function arguments, `self` is not a reserved keyword so you may use another name if it's more appropriate.

Lastly, realize that unlike a method in an object-oriented language, a trait cannot mutate the original item. It can still do internal reassignment, but it cannot do actual mutation.

### Non-aliased traits

To implement a trait without an alias, you can simply say `extend [item]`:

```buri
MinHeap = [Num]

# implement a named trait
extend MinHeap with Pushable(Num):
    push(self, item) =>
        self:push(item)
        -- add logic to ensure that the items is still a heap
        return self

# implement a nameless trait
extend MinHeap:
    getMin(self) =>
        return self[0] -- returns Option(Num)

heap: Heap = []
minValue = heap:push(1)
    :push(2)
    :push(20)
    :push(3)
    :push(-1)
    :getMin() -- -1
```

By combining both named and nameless traits, it allows you to quickly scaffold highly reusable code.

### Trait bounds

TODO: write this, how can we specify traits as arguments

### Traits for platforms

TODO: write this, how can we specify traits for platforms

## Traits and structural typing

### Inferring trait definitions

Consider the following function:

```buri
getName = (item) => item:calcName()
```

Notice that using ":" to call a means that function is a trait, not just an arbitrary function.

This means item has the "trait" of something with the "calcName" function. More succinctly, `item` has the following type:

```buri
t extends {
    calcName: (self) => *,
}
```

Now imagine the following function:

```buri
squareSums = (items) => items:map((x) => x * x):reduce((x, total, 0) => x + total)
```

Here, items has the "trait" of something with the "map" function that accepts a function that accepts and returns a Num, where the "map" returns something that has the trait of something with the "reduce" function that accepts a function with three Num arguments and returns a Num, and this "reduce" function returns anything. Or again, succinctly it would be the following type:

```buri
t extends {
    map: (self, (Num) => Num) => u extends {
        reduce: (self, (Num, Num, Num) => Num) => *,
    },
}
```

Of course if we just define "items" to be a list of Num, then it can dramatically simplify the types as now everything has a definite type. But still, the function is fully typed even without the type annotation.

```buri
squareSums = (items: [Num]) => items:map((x) => x * x):reduce((x, total, 0) => x + total)
```

This system allows for determining traits with structural typing.

### adding traits to a variable

When "defining" traits, there are two things we could be talking about: a) creating trait aliases (e.g., Pushable, Iterable, etc.), or b) adding a new function to a record's trait. This message is about "b".

The key to this is that traits are added nominally. Let's say you have the following:

```buri
Person = {
    name: Str,
    age: Nat, -- natural number
}

extend Person:
    celebrateBirthday = (self) => { ...self, age: self.age + 1 }

me = { name: "Nick", age: 24 }
olderMe = me:celebrateBirthday() -- error, "me" doesn't have the trait "celebrateBirthday"
```

Why do we get this error? Well, because "me" is of type { name: Str, age: Nat }, not "Person". Only "Person" has the desired trait for "celebrateBirthday". However, if we define "me" to be of type "Person", things will work.

```buri
me: Person = { name: "Nick", age: 24 }
olderMe = me:celebrateBirthday() -- compiles
```

This prevents the need of solving the halting problem. Now there are a few edge cases. What happens in the following scenarios?

```buri
getAgeNextYear = (person: Person) => person:celebrateBirthday().age
me = { name: "Nick", age: 24 }
olderMeAge = getAgeNextYear(me) -- should this compile?
```

I'd say this should still compile. We can say that because getAgeNextYear requires a person, we can infer the original type of "me" to be a person. And hence it would have the trait. Now what about the following scenario?

```buri
Student = { name: Str, age: Nat }
Person = -- as above, including celebrateBirthday trait
getAgeNextYear = (person: Person) => person:celebrateBirthday().age
me: Student = { name: "Nick", age: 24 }
olderMeAge = getAgeNextYear(me) -- should this compile?
```

I'd say this should not run. Student doesn't have the trait for "celebrateBirthday" as required by "getAgeNextYear". Now what about the following scenario:

```buri
Student = { name: Str, age: Nat }
Person = { name: Str, age: Nat }

extend Person:
    celebrateBirthday = (self) => { ...self, age: self.age + 1 }

extend Student:
    celebrateBirthday = (self) => { name: "Older " ++ self.name, age: self.age + 1 }

getAgeNextYear = (person: Person) => person:celebrateBirthday().age
me: Student = { name: "Nick", age: 24 }
olderMeAge = getAgeNextYear(me) -- should this compile?
```

If we ignore the types, this would definitely compile and run. If we used nominal typing, it wouldn't. However, this would work with structural typing. And since Buri uses structural typing, I'd argue this should compile, raise a linting error, and olderMeAge = { name: "Older Nick", age: 25 }.

### Default traits

Several items can have default traits. For instance, lists can have the :push(), :pop() and other traits that one would expect. To make updating records nicer (and admittedly create nicer protobuff support), records would have default "set" traits for every item.

```buri
me = { name: "Nick", age: 24 }
olderMe = me:setAge(24)
```

Why would you want to do this? Mainly to help with function chaining and make the builder pattern available by default.

```buri
Person = { name: Str = "Jane Doe", age: Nat = 0 }
me: Person = {} -- default fields are filled by default
me:setName("Nick"):setAge(24)
```

Through trait definitions (see text #2), you can override the default traits.

```buri
Person = { name: Str, age: Nat }

extend Person:
    setAge = (self, age) =>
        if age > self.age then
            self:setName("Older " ++ self.name)
        end
        self.age = age
        return self

me: Person = { name: "Nick", age: 24 }
olderMe = me:setAge(25) -- { name: "Older Nick", age: 25 }
```

Of course overriding in this case could lead to unpredictable results and should not be used often. But there are legitimate reasons to use it (e.g., overriding "push" for a Heap, protobuffs "oneof" feature, etc.).

```buri
type Person = { name: Str, age: Num }
type Student = { name: Str, age: Num }
type RealPerson = Person
// { age: Num, name: Str }
// { name: Str, age: Num }
// Person

aaron = {
    name: "Aaron",
    age: 314,
}
nick = aaron:setName("Nick")
getThing = (person: Person) => ...
getThing(aaron)

getStudentAttendanceHistory = (student: Student) => ...
getStudentAttendanceHistory(aaron) -- error!

a = { value: 1 }
a = a:setValue(13)

myList = [1, 2, 3, 4, 5, 6, 7, 8, 9]
secondList = myList:set(3, 210) -- [1, 2, 3, 210, 5, 6, 7, 8, 9]
myList = myList:push(10)
thirdList = myList:set(4, 210) -- [1, 2, 3, 210, 5, 6, 7, 8, 9]
console.log(secondList:length())

// old Buri
add = (a, b) => a + b
square = (a) => a ** 2

item = square(add(1, 2)) -- 9
nine = add(1, 2):square() -- 9

squareAdd = (a, b) =>
    where add(a, b) is
        #err(err) do return err
        #ok(value) do square(value)
        value do square(value)


// C

struct secondList {
    int* originalArray;
    int valueAt3;
}

struct myList(2) {
    int* myListOriginal;
    int appendedValue;
}

struct thirdList {
    int* myListOriginal;
    int appendedValue;
    int valueAt4;
}

// Rust

let mut my_vec = Vec::new(); // Vec<usize>
my_vec.push(4);
my_vec.push("hello"); // error, my_vec has type Vec<usize>

// person: extends({ age: a })
getAge = (person) => person.age

// person: extends({ setAge: (self, age) => self })
setAge = (person, age) => person:setAge(age)

Colors = #red | #green | #blue | *

// (Num) => #red | #green | #blue
getColor = (thing) =>
    when thing is
        0 do #green,
        1 do #blue,
        _ do #red,

// getColorType(a extends #red | #green | #blue | #yellow | #purple, #magenta, b extends | #primary | #secondary): (a) => b
getColorType = (color) =>
    when color is
        #red | #green | #blue do #primary,
        #yellow | #purple | #magenta do #secondary,
        _ do color

GetColorType(a extends #red ...) = (a) => Result(#primary | #secondary, a)
// (extends(#red | #green | #blue ...)): (a) => extends( #ok(#primary | #secondary) )
getColorType = (color) =>
    when color is
        #red | #green | #blue do #ok(#primary),
        #yellow | #purple | #magenta do #ok(#secondary),
        _ do #err(color)

wrapper = (func: GetColorType, color) => func(color)
wrapper(getColorType, #green) -- type error

Something = extends({
    ..,
    ..,
    ..,
    ..,
    ..,
    ..,
    ..,
    ..,
    ..,
    ..,
})

myFunc = (arg: Something) => ...


// TypeScript

const hello: ("nick" | "aaron") = "aaron"
---
type Hello = "nick" | "aaron"
const hello: Hello = "aaron"

// Swift

enum Color {
    case red;
    case green;
    case blue;
}

let color = Color.red;
let red: Color = .red;

struct Person = {
    age: Int,
    name: String,
}

let person: Person = {
    age: 24,
    name: "Nick",
}

let hello = person.age;
let world = Color.red;


// await: (Task, Task?) => Task
result1 = await(hello) -- hello: () => #ok(Bool) | #err(#fileNotFound | #undecodableFile)
result2 = await(world, result1) -- world: (Bool) => -- #ok(Str) | #err(#404 | #500)
fullTask = await(anotherTask(result2)) -- anotherTask: (Str) => #ok(Num) | #err(#quantumVoodooMagic)

Task.run(task:compose(task2):compose(task3))

Result(ok, err) = #ok(ok) | #err(err)
Task.run(fullTask, (result: Result(Num, #fileNotFound | #undecodableFile | #404 | #500 | #quantumVoodooMagic)) =>
    when result is
        #ok(_) do console.log(success),
        #err(#fileNotFound) do --something else,
)

result = task1()?:task2()?:task3()

Friend = {
    name: Str,
    age: Nat,
}

Enemy = {
    name: Str,
    age: Nat,
}

extend Friend:
    sayHello = @impure (self) =>
        console.log(self.name ++ " says hello!")
        return self

    increaseAge = (self) =>
        self:setAge(self.age + 1)

    shouldDoSomething = (self) => #false

extend Enemy:
    sayHello = (self) =>
        -- enemies don't say "hello"
        self

    increaseAge = (self) =>
        self:setAge(self.age + 1000)

    shouldDoSomething = (self) => #true

inner = (item) =>
    if item:shouldDoSomething do
        item:sayHello()?:increaseAge()
    else
        #err(#itemSaidNothingShouldBeDone)

hello = @impure () => -- ...

acceptsFunction = @impure (shouldRun: Bool, func: @impure) => if shouldRun do func()

main = @impure () =>
    aaron: Friend = { name: "Aaron", age: 2 }
    redSkull: Enemy = { name: "Red Skull", age: 10000000 }
    inner(aaron)
    inner(redSkull)
    acceptsFunction(hello)

Color =
    | #red = 0
    | #green = 1
    | #blue = 2

Color = #red = "fire" | #green = "earth" | #blue = "water"

nickShirt: Color = #red
console.log(nickShirt) -- "fire"

// Problem statement
1. Is this value apart of a specific subset of a tag union or not?
2. Doing manipulations of tag values.

      | hash map
       | vector
|||| ||| -> data structure
0000 0000
        ^ immutable bit (1 is immutable)

DataType =
    | #mutableVector @mutable
    | #immutableVector @analog(#mutableVector)
    | #mutableHashMap @mutable @analog(#immutableHashMap)
    | #immutableHashMap

myThing: DataType = ...

myThing.isMutable()

extend DataType:
    isMutable = (self) =>
        self:asInt() & 1 == 1

// TypeScript
type Mutability = "immutable" | "mutable"
type Container = "vector" | "hashMap"
type DataType = `${Mutability}-${Container}` // "immutable-vector" | "mutable-vector" | "immutable-hashMap" | "mutable-hashMap"

DataType = {
    isMutable: bool,
    type: #vector | #hashMap | #array -- ...
}

beforeEach(() => {

})

describe("this is the test block", () => {


    it("this is a single test description", () => {
        expect(true).toBe(true)
        expect(true).toBeTruthy(true)
        expect(true).toEqual(true)
        expect({a: true, b: "hello"}).toEqual({a: true, b: "hello"})
        expect(true).toMatch("hello")

        expect({a: true, b: "hello"}).toBeTrue()
    })

    beforeEach(() => {

    })
})

nthFibbonaccies = (n) => -- ... [Num, Num, Num]

fiveFibs = @inline nthFibbonaccies(5)

expect = (item) =>
    where typeof item is
        struct do #expectStruct(item)
        tagUnion do #expectTagUnion(item)
        result do #expectResult(item)
        boolean do #expectBoolean(item)


ExpectReturnValue(t) = {
    value: t,
}

expect = (t) => ExpectReturnValue { value: t }

extend ExpectReturnValue with Pushable:
    push = (self) => -- ...

extend ExpectReturnValue(extends({})):


    toExactlyEqual = (self) => -- ... test if the two structs are sldkjflsjdfa;ldkfjsdf

extend ExpectReturnValue(Bool):
    toBeFalse = (self) => self == #false

extend ExpectReturnValue(t):
    toEqual = (self, other) => self == other

    identity = (self) => self.value

wrapper = (item) =>
    expect(item):identity()



fn my_func<Ok, Err>() -> Result<Ok, Err> {
    let ok_value = task()?;

}
```
