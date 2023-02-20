import { five } from "@tests/js/valid/if/chain.mjs"
import { expect, it } from "bun:test"

it("a chain of if statements", () => {
    expect(five).toBe(5)
})
