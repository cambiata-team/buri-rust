import {
    BeightModFive,
    BeightModNegativeFive,
    BfiveModEight,
    BfiveModNegativeEight,
    BnegativeEightModFive,
    BnegativeEightModNegativeFive,
    BnegativeFiveModEight,
    BnegativeFiveModNegativeEight,
} from "@tests/js/valid/integers/modulo.mjs"
import { describe, expect, it } from "bun:test"

describe("8 and 5", () => {
    it("8 % 5 == 3", () => {
        expect(BeightModFive.valueOf()).toBe(3)
    })

    it("8 % -5 == -2", () => {
        expect(BeightModNegativeFive.valueOf()).toBe(-2)
    })

    it("-8 % 5 == 2", () => {
        expect(BnegativeEightModFive.valueOf()).toBe(2)
    })

    it("-8 % -5 == -3", () => {
        expect(BnegativeEightModNegativeFive.valueOf()).toBe(-3)
    })
})

describe("5 and 8", () => {
    it("5 % 8 == 5", () => {
        expect(BfiveModEight.valueOf()).toBe(5)
    })

    it("5 % -8 == -3", () => {
        expect(BfiveModNegativeEight.valueOf()).toBe(-3)
    })

    it("-5 % 8 == -5", () => {
        expect(BnegativeFiveModEight.valueOf()).toBe(3)
    })

    it("-5 % -8 == -5", () => {
        expect(BnegativeFiveModNegativeEight.valueOf()).toBe(-5)
    })
})
