import {
    BfiveDivideFive,
    BfiveMinusFive,
    BfiveModuloFive,
    BfivePlusNegativeFive,
    BfourDivideFour,
    BfourMinusFour,
    BfourModuloFour,
    BfourPlusNegativeFour,
    BnegativeFiveDivideNegativeFive,
    BnegativeFiveMinusNegativeFive,
    BnegativeFiveModuloNegativeFive,
    BnegativeFivePlusFive,
    BnegativeFourDivideNegativeFour,
    BnegativeFourMinusNegativeFour,
    BnegativeFourModuloNegativeFour,
    BnegativeFourPlusFour,
} from "@tests/js/valid/integers/inverse.mjs"
import { describe, expect, it } from "bun:test"

describe("addition has inverses", () => {
    it("5 + -5 == 0", () => {
        expect(BfivePlusNegativeFive.valueOf()).toBe(0)
    })

    it("-5 + 5 == 0", () => {
        expect(BnegativeFivePlusFive.valueOf()).toBe(0)
    })

    it("4 + -4 == 0", () => {
        expect(BfourPlusNegativeFour.valueOf()).toBe(0)
    })

    it("-4 + 4 == 0", () => {
        expect(BnegativeFourPlusFour.valueOf()).toBe(0)
    })
})

describe("subtraction has inverses", () => {
    it("5 - 5 == 0", () => {
        expect(BfiveMinusFive.valueOf()).toBe(0)
    })

    it("-5 - -5 == 0", () => {
        expect(BnegativeFiveMinusNegativeFive.valueOf()).toBe(0)
    })

    it("4 - 4 == 0", () => {
        expect(BfourMinusFour.valueOf()).toBe(0)
    })

    it("-4 - -4 == 0", () => {
        expect(BnegativeFourMinusNegativeFour.valueOf()).toBe(0)
    })
})

describe("division has inverses", () => {
    it("5 / 5 == 1", () => {
        expect(BfiveDivideFive.valueOf()).toBe(1)
    })

    it("-5 / -5 == 1", () => {
        expect(BnegativeFiveDivideNegativeFive.valueOf()).toBe(1)
    })

    it("4 / 4 == 1", () => {
        expect(BfourDivideFour.valueOf()).toBe(1)
    })

    it("-4 / -4 == 1", () => {
        expect(BnegativeFourDivideNegativeFour.valueOf()).toBe(1)
    })
})

describe("modulo has inverses", () => {
    it("5 % 5 == 0", () => {
        expect(BfiveModuloFive.valueOf()).toBe(0)
    })

    it("-5 % -5 == 0", () => {
        expect(BnegativeFiveModuloNegativeFive.valueOf()).toBe(0)
    })

    it("4 % 4 == 0", () => {
        expect(BfourModuloFour.valueOf()).toBe(0)
    })

    it("-4 % -4 == 0", () => {
        expect(BnegativeFourModuloNegativeFour.valueOf()).toBe(0)
    })
})
