import { booleanFalse, booleanTrue } from "@tests/js/valid/tags/declaration.mjs"
import { expect, it } from "bun:test"

it("compiler boolean true", () => {
    expect(booleanTrue).toBe(true)
})

it("compiler boolean false", () => {
    expect(booleanFalse).toBe(false)
})
