import { four, three } from "@.buri/dist/tests/js/valid/functions/arguments.mjs"
import { describe, expect, it } from "bun:test"

describe("add = (a, b) => a + b", () => {
    it("add(1, 2)", () => {
        expect(three.valueOf()).toEqual(3)
    })

    it("add(3, 1)", () => {
        expect(four.valueOf()).toEqual(4)
    })
})
