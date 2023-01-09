# Platforms

> **Note:** this page is still under construction. There are many decisions yet to make in regards to how platforms should work.

Buri has a very similar distinction between platforms and applications to [Roc](https://roc-lang.org/). By separating the two, it allows Buri developers to create highly reusable and secure code. This has several unique benefits:

- Any `pure` function can be compiled to any platform (e.g., JavaScript, macOS binary, Windows binary, CLI, etc.)
- etc.

One way you can think of this is that developers create applications that are then built and run by platforms.

## Events

Buri doesn't have the concept of main function. Instead, Buri uses events. And every application *must* export a list of events to the platform.

```buri
main = (event) => {
    -- ...
}

handleKeyPress = (event) => {
    -- ...
}

export events = [Main(main), KeyPress(handleKeyPress)]
```

### Syntax

In the main file, export a variable named `events` that is a list of events:

```buri
export events = [ ... ]
```

Each event is a tag where the value of each tag is a function.

The tag name allows the platform to distinguish between multiple events. For instance, in this case:

```buri
main = (event) => {
    -- ...
}

handleKeyPress = (event) => {
    -- ...
}

export events = [Main(main), KeyPress(handleKeyPress)]
```

The `Main` tag is used to denote which function should run when the application starts. The `KeyPress` tag is used to denote which function should run when a key is pressed.

Then, each function is run with a populated `event` record. The exact fields this record contains depends on the platform and the event.

### Multiple platforms

```buri
import env from 'buri/env'

export events = when env is
    Web do [ ... ],
    Desktop do [ ... ],
    Mobile do [ ... ],
    Cli do [ ... ],
```
