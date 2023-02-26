import { Bfive, Bfour } from "@tests/js/valid/if/block.mjs"
import { expect, it } from "bun:test"

it("can have multiple expressions inside the #true branch", () => {
    expect(Bfive.valueOf()).toBe(5)
})

it("can have multiple expressions inside the #false branch", () => {
    expect(Bfour.valueOf()).toBe(4)
})
