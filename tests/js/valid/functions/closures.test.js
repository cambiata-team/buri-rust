import { addN, three } from "@tests/js/valid/functions/closures.mjs"
import { describe, expect, it } from "bun:test"

describe("addN", () => {
    it("add 1", () => {
        const add1 = addN(1)
        expect(add1(1).valueOf()).toEqual(2)
        expect(add1(2).valueOf()).toEqual(3)
    })

    it("add 2", () => {
        const add2 = addN(2)
        expect(add2(1).valueOf()).toEqual(3)
        expect(add2(2).valueOf()).toEqual(4)
    })
})

describe("three", () => {
    it("should return 3", () => {
        expect(three().valueOf()).toEqual(3)
    })
})
