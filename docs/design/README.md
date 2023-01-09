# Design

1. [Commandments](commandments.md)
2. [Expressions](expressions.md)
3. [Code blocks](code-blocks.md)
4. [Type system](type-system.md)
   1. [Type inference](type-inference.md)
   2. [Type identifiers](type-identifiers.md)
5. [Variable declaration](variable-declaration.md)
6. [Primitives](primitives.md)
7. [Collections](collections.md)
   1. [Records](records.md)
   2. [Tags](tags.md)
   3. [Lists](lists.md)
8. [Functions](functions.md)
9. [Internal Reassignment](internal-reassignment.md)
10. [Control flow](control-flow.md)
11. [Optional chaining](optional-chaining.md)
12. [Modules (importing / exporting)](modules.md)
13. [Platforms](platforms.md)
14. [Deprecation](deprecation.md)
15. [Comments](comments.md)
16. [Syntax cheatsheet](syntax-cheatsheet.md)

## Main objective

The main objective of Buri is to increase developer velocity. With every decision we make, we should ask ourselves if it increases developer velocity. If it does not, we should not do it.

## Guiding principles

### Simple and consistent is better

A language with fewer features is easier to learn, more powerful to use, and much more predictable. Always consider the impact of adding new features and how that effects the overall usability and maintainability of code using this language. Use consistent syntax as much as possible.

One heuristic we can use to determine the simplicity of Buri is to look at the amount of documentation we'd need to write. If we need to write a lot of documentation, it's probably not simple enough.

### Declarative over imperative

As much as possible, the language should move towards declarative features. For instance, pattern matching over if-else chains, and functional over object-oriented with side effects.

### Type-sound

All code written in this language should be type-sound. For instance, if variable is declared as an `integer`, it must always be an integer at runtime. This is vital. This language compiles directly into JavaScript, and JavaScript does not do runtime type-checking. If this language is type-sound, we can dramatically reduce the compiled size, leading to extremely fast and correct JavaScript. This can become especially tricky when interfacing with JavaScript APIs, but it's vital to still be type-sound.

### Fast and correct

The JavaScript output should be fast and correct. There should be no runtime errors. By designing a language that guides developers into using good coding practices, we can dramatically optimize the code. We will no longer need runtime type checking since we can check all types at compile time. We can reduce the code output as we can guarantee certain code paths will never run. We can guide developers to handling errors correctly, leading to never creating flakey products.

## Reference languages

Here are a list of languages to reference:

- **[JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript)**: obviously reference JavaScript since the language should feel as similar to JavaScript as possible.
- **[TypeScript](https://www.typescriptlang.org/)**: the most popular typed version of JavaScript. Reference this when deciding the syntax for using types.
- **[Rust](https://www.rust-lang.org/)**: the most loved language according to every StackOverflow survey in the last 6 years. Obviously people like it, so reference it's syntax and language features. Especially reference how it eliminates runtime errors.
- **[ReScript](https://rescript-lang.org/)**: The best soundly-typed version of compile-to-javascript languages. Reference this for it's language features and compiled output, but not it's syntax.
- **[Elm](https://elm-lang.org/)**: A functional language that compiles to JavaScript and used for building websites. Reference how it uses functional paradigms.
- **[Roc](https://roc-lang.org/)**: An experimental functional programming language. Reference for it's simplicity and application/platform feature.

## Todo

- [x] Solidify primitives
  - [x] How numbers work across platforms
  - [x] Number overflow/underflow
  - [x] String definitions
  - [x] String interpolation
- [ ] Effects (i.e., tasks)
  - [ ] Async/await
- [x] Code blocks
  - [x] Curly-brace vs indentation
- [ ] Operations
  - [x] Boolean
  - [x] Numeric
    - [x] add, subtract…
    - [x] Bitwise
  - [x] String
- [x] Collections
  - [x] Records
  - [x] Tags
    - [x] Scoped tags? no
  - [x] Tuples
  - [x] Lists (arrays)
    - [x] Should accessing elements in an array return a Result or Option?
    - [x] List type notation
  - [ ] Maps
  - [ ] Sets
  - [x] Destructuring
- [x] Functions
  - [x] Optional arguments
  - [x] Partial function composition (i.e., currying)
- [x] Control flow
  - [x] If/else
  - [x] when
  - [x] Ternary operator (not included)
- [x] Seamless integration with protocol buffers
- [ ] Standard library
  - [ ] Strings
    - [ ] Pattern matching / regex
  - [ ] Data structures
    - [ ] BST
    - [ ] Heap
    - [ ] Red-black tree
    - [ ] AVL tree
    - [ ] Queue (and deque)
- [ ] Concurrency
- [ ] Platforms
  - [x] main function
  - [x] syntax
  - [ ] inheritance between multiple platforms
  - [ ] Deploying to multiple platforms
  - [ ] dependencies and platforms
- [ ] Resolve how it works with declarative web dev front-end frameworks
- [ ] Testing
- [ ] Documentation comments
- [x] Importing/exporting
- [ ] Finishing this todo list
