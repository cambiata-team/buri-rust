# Commandments

These commandments describe what a user should and should not be able to do.

1. Users should be able to know if a specific piece of code (e.g., function) has any side effects without reading through the entire source code or any of it's dependencies.
2. A user should be able to write code once and build it for any platform (e.g., desktop applications, clis, web browsers, web assembly, etc.). Likewise, users should be able to build 3rd-party libraries and be able to use them in their own project, no matter which platform their project is for.
3. Users may not initialize a variable without instantiating it with a value.
4. There shall not be any runtime errors (aside from integer overflow/underflow).
5. A user should be able to write an entire program without needing to annotate any types while still being type-sound.
6. Users should not need to distinguish between references, pointers, and the underlying data.
7. Users should not implicitly convert data between different types. All type conversions should be explicit to remove the possibility of accidental conversions.
