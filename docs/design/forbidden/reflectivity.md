# Reflectivity

Many languages have a concept of "reflectivity" which allows you to inspect the structure of a program at runtime. For instance, in JavaScript, you can use a variable to access an object's property as shown in the following code snippet:

```js
let jane = {
  name: "Jane",
  age: 30,
}
let property = "name"
let janesName = jane[property]
```

While this may seem like a useful feature, it the following huge drawbacks:

- It makes it impossible to statically analyze the code. For instance, if you have a typo in the property name, the compiler will not be able to catch it.
- It makes dead code elimination impossible. For instance, if you have a function that is only called with a specific property name, the compiler will not be able to remove the other branches.
- Accessing properties dynamically is substantially slower than accessing them statically.
- It opens up the possibility for malicious code to access private properties when Buri is compiled other languages (such as JavaScript). This is a huge security risk and you cannot catch this in your unit tests.

In short, it increases compile size, decreases performance, weakens static analysis, and opens up untestable security vulnerabilities.

If you need to access properties dynamically, use a hash map.
