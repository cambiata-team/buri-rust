# Modules (importing / exporting)

Every Buri file is a module, and every module is a Buri file. It is not possible to create multiple modules inside a single file.

## Exporting

To export anything, use the `@export` decorator:

```buri
@export
Color = #red | #green | #blue

@export
toString = (color) =>
    when color is
        #red => "red"
        #green => "green"
        #blue => "blue"

@export
hello = 'hello world'
```

Any value or type can be exported. However, you can only export during a variable declaration. So the following is invalid:

```buri
@export
'hello world'
```

All exports must be at the top level of the file. For instance, you cannot export a value from inside a function:

```buri
hasExport = (arg) =>
    @export
    Color = #red | #green | #blue -- compile error
    -- ...
```

## Importing

Imports have the following format:

```buri
import Color, toString, hello from './colors.buri'
```

Once imported, you can use the imported values as if they were local variables.

```buri
import Color, toString, hello from './colors.buri'

color: Color = #green
green = toString(color)
```

The exact names of the imported values are determined by the export. If a file has `@export hello`, you will always use the name `hello` to import it.

Imports must be at the top level of the file (e.g., you can't have an import inside a function). The following is invalid.

```buri
usesImport = (arg) =>
    import Color from './colors.buri' -- compile error
    -- ...
```

However, while imports do not need to be at the beginning of a file, they are recommended to be at the beginning of the file and the Buri formatter will place them there.

### Renaming imports

Sometimes you may need to rename an import to avoid naming conflicts. For this, simply use `as` and provide the new name:

```buri
import Color as PrimaryColor from './colors.buri'
```

If you need to rename multiple imports from a single file, use `as` for each:

```buri
import
    Color as PrimaryColor,
    toString as primaryColorToString,
    hello as helloWorld,
    from './colors.buri'
```

> **Note:** You may add an optional comma after the last item in an import statement. That way, if the import statement spans multiple lines, you can add the comma and eliminate unnecessary diffs in version control.
