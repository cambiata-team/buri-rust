import {
    eightDivideNegativeThree,
    eightDivideNegativeTwo,
    eightDivideThree,
    eightDivideTwo,
    negativeEightDivideNegativeThree,
    negativeEightDivideNegativeTwo,
    negativeEightDivideThree,
    negativeEightDivideTwo,
} from "@tests/js/valid/integers/division.mjs"
import { describe, expect, it } from "bun:test"

describe("use normal division rules when dividing by a divisor", () => {
    it("8 / 2 == 4", () => {
        expect(eightDivideTwo.valueOf()).toBe(4)
    })

    it("-8 / 2 == -4", () => {
        expect(negativeEightDivideTwo.valueOf()).toBe(-4)
    })

    it("8 / -2 == -4", () => {
        expect(eightDivideNegativeTwo.valueOf()).toBe(-4)
    })

    it("-8 / -2 == 4", () => {
        expect(negativeEightDivideNegativeTwo.valueOf()).toBe(4)
    })
})

describe("dividing by non-divisors round towards zero", () => {
    it("8 / 3 == 2", () => {
        expect(eightDivideThree.valueOf()).toBe(2)
    })

    it("-8 / 3 == -2", () => {
        expect(negativeEightDivideThree.valueOf()).toBe(-2)
    })

    it("8 / -3 == -2", () => {
        expect(eightDivideNegativeThree.valueOf()).toBe(-2)
    })

    it("-8 / -3 == 2", () => {
        expect(negativeEightDivideNegativeThree.valueOf()).toBe(2)
    })
})
