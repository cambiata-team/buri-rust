import { b } from "@tests/js/valid/declarations/declarations.mjs"
import { describe, expect, it } from "bun:test"

it("a variable can be declared equal to another variable", () => {
    expect(b.valueOf()).toBe("name")
})