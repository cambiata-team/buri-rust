import {
    BeightDivideNegativeThree,
    BeightDivideNegativeTwo,
    BeightDivideThree,
    BeightDivideTwo,
    BnegativeEightDivideNegativeThree,
    BnegativeEightDivideNegativeTwo,
    BnegativeEightDivideThree,
    BnegativeEightDivideTwo,
    BzeroDivideNegativeTwo,
    BzeroDivideTwo,
} from "@tests/js/valid/integers/division.mjs"
import { describe, expect, it } from "bun:test"

describe("use normal division rules when dividing by a divisor", () => {
    it("8 / 2 == 4", () => {
        expect(BeightDivideTwo.valueOf()).toBe(4)
    })

    it("-8 / 2 == -4", () => {
        expect(BnegativeEightDivideTwo.valueOf()).toBe(-4)
    })

    it("8 / -2 == -4", () => {
        expect(BeightDivideNegativeTwo.valueOf()).toBe(-4)
    })

    it("-8 / -2 == 4", () => {
        expect(BnegativeEightDivideNegativeTwo.valueOf()).toBe(4)
    })
})

describe("dividing by non-divisors round towards zero", () => {
    it("8 / 3 == 2", () => {
        expect(BeightDivideThree.valueOf()).toBe(2)
    })

    it("-8 / 3 == -2", () => {
        expect(BnegativeEightDivideThree.valueOf()).toBe(-2)
    })

    it("8 / -3 == -2", () => {
        expect(BeightDivideNegativeThree.valueOf()).toBe(-2)
    })

    it("-8 / -3 == 2", () => {
        expect(BnegativeEightDivideNegativeThree.valueOf()).toBe(2)
    })
})

describe("zero divide anything is zero", () => {
    it("0 / 2 == 0", () => {
        expect(BzeroDivideTwo.valueOf()).toBe(0)
    })

    it("0 / -2 == 0", () => {
        expect(BzeroDivideNegativeTwo.valueOf()).toBe(0)
    })
})
