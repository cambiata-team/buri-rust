import { BaddN, Bthree } from "@tests/js/valid/functions/closures.mjs"
import { describe, expect, it } from "bun:test"

describe("addN", () => {
    it("Badd 1", () => {
        const add1 = BaddN(1)
        expect(add1(1).valueOf()).toEqual(2)
        expect(add1(2).valueOf()).toEqual(3)
    })

    it("Badd 2", () => {
        const add2 = BaddN(2)
        expect(add2(1).valueOf()).toEqual(3)
        expect(add2(2).valueOf()).toEqual(4)
    })
})

describe("Bthree", () => {
    it("should return 3", () => {
        expect(Bthree().valueOf()).toEqual(3)
    })
})
