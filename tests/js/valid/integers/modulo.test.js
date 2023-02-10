import {
    eightModFive,
    eightModNegativeFive,
    fiveModEight,
    fiveModNegativeEight,
    negativeEightModFive,
    negativeEightModNegativeFive,
    negativeFiveModEight,
    negativeFiveModNegativeEight,
} from "@tests/js/valid/integers/modulo.mjs"
import { describe, expect, it } from "bun:test"

describe("8 and 5", () => {
    it("8 % 5 == 3", () => {
        expect(eightModFive.valueOf()).toBe(3)
    })

    it("8 % -5 == -2", () => {
        expect(eightModNegativeFive.valueOf()).toBe(-2)
    })

    it("-8 % 5 == 2", () => {
        expect(negativeEightModFive.valueOf()).toBe(2)
    })

    it("-8 % -5 == -3", () => {
        expect(negativeEightModNegativeFive.valueOf()).toBe(-3)
    })
})

describe("5 and 8", () => {
    it("5 % 8 == 5", () => {
        expect(fiveModEight.valueOf()).toBe(5)
    })

    it("5 % -8 == -3", () => {
        expect(fiveModNegativeEight.valueOf()).toBe(-3)
    })

    it("-5 % 8 == -5", () => {
        expect(negativeFiveModEight.valueOf()).toBe(3)
    })

    it("-5 % -8 == -5", () => {
        expect(negativeFiveModNegativeEight.valueOf()).toBe(-5)
    })
})
