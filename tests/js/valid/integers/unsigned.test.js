import { one, two } from "@tests/js/valid/integers/unsigned.mjs"

import { expect, it } from "bun:test"

it("a literal with the value of 1 should be equal to 1", () => {
    expect(one).toBe(1)
})

it("a literal with the value of 2 should be equal to 2", () => {
    expect(two).toBe(2)
})
