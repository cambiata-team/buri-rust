import { Ba, Bb } from "@tests/js/valid/comments/semantic-invariance.mjs"
import { expect, it } from "bun:test"

it("The presence of a comment does not change the value of an expression", () => {
    expect(Ba).toBe(Bb)
})
