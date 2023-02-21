import {
    fiveDivideFive,
    fiveMinusFive,
    fiveModuloFive,
    fivePlusNegativeFive,
    fourDivideFour,
    fourMinusFour,
    fourModuloFour,
    fourPlusNegativeFour,
    negativeFiveDivideNegativeFive,
    negativeFiveMinusNegativeFive,
    negativeFiveModuloNegativeFive,
    negativeFivePlusFive,
    negativeFourDivideNegativeFour,
    negativeFourMinusNegativeFour,
    negativeFourModuloNegativeFour,
    negativeFourPlusFour,
} from "@tests/js/valid/integers/inverse.mjs"
import { describe, expect, it } from "bun:test"

describe("addition has inverses", () => {
    it("5 + -5 == 0", () => {
        expect(fivePlusNegativeFive.valueOf()).toBe(0)
    })

    it("-5 + 5 == 0", () => {
        expect(negativeFivePlusFive.valueOf()).toBe(0)
    })

    it("4 + -4 == 0", () => {
        expect(fourPlusNegativeFour.valueOf()).toBe(0)
    })

    it("-4 + 4 == 0", () => {
        expect(negativeFourPlusFour.valueOf()).toBe(0)
    })
})

describe("subtraction has inverses", () => {
    it("5 - 5 == 0", () => {
        expect(fiveMinusFive.valueOf()).toBe(0)
    })

    it("-5 - -5 == 0", () => {
        expect(negativeFiveMinusNegativeFive.valueOf()).toBe(0)
    })

    it("4 - 4 == 0", () => {
        expect(fourMinusFour.valueOf()).toBe(0)
    })

    it("-4 - -4 == 0", () => {
        expect(negativeFourMinusNegativeFour.valueOf()).toBe(0)
    })
})

describe("division has inverses", () => {
    it("5 / 5 == 1", () => {
        expect(fiveDivideFive.valueOf()).toBe(1)
    })

    it("-5 / -5 == 1", () => {
        expect(negativeFiveDivideNegativeFive.valueOf()).toBe(1)
    })

    it("4 / 4 == 1", () => {
        expect(fourDivideFour.valueOf()).toBe(1)
    })

    it("-4 / -4 == 1", () => {
        expect(negativeFourDivideNegativeFour.valueOf()).toBe(1)
    })
})

describe("modulo has inverses", () => {
    it("5 % 5 == 0", () => {
        expect(fiveModuloFive.valueOf()).toBe(0)
    })

    it("-5 % -5 == 0", () => {
        expect(negativeFiveModuloNegativeFive.valueOf()).toBe(0)
    })

    it("4 % 4 == 0", () => {
        expect(fourModuloFour.valueOf()).toBe(0)
    })

    it("-4 % -4 == 0", () => {
        expect(negativeFourModuloNegativeFour.valueOf()).toBe(0)
    })
})
