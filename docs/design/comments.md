# Comments

Comments are sections of the code that are not executed or parsed. They will be completely removed from the compiled output.

Comments start with a `--` and continue to the end of the line.

```buri
-- I am a single line comment
a = "hello" -- I am a single line comment that starts after some code
```

Buri does not have multi-line comments.

## Documentation comments

Documentation comments must appear on their own line, and start with `-!`.

```buri
-! This is a documentation comment
-!
-! Because I chained together several documentation comments,
-! they are considered to be a single comment.
```

Inside a documentation comment, you can use any valid Markdown. It will then be used to generate HTML documentation when you publish a package, in your IDE as you hover over a variable, or any other time that it could be useful.

### Documentation Comments as Tests

Adding code examples to your documentation comments is a great way to show others how to use your code. But nothing is worse than examples that don't work because the comment is out of date. So any code examples you write in your documentation comments will be treated as a test when you run your tests.

```buri
-! This is a function that does something.
-!
-! ```
-! output = hello("world")
-! @assert(output == "hello world")
-! ```
```

If you write a fenced code block for another language, it will not be treated as a test.

```buri
-! This is a function that does something.
-!
-! ```md
-! I am a markdown code block and am not a test.
-! ```
```
