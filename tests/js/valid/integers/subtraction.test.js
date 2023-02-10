import {
    negativeOneMinusNegativeTwo,
    negativeOneMinusTwo,
    negativeThreeMinusFour,
    negativeThreeMinusNegativeFour,
    oneMinusNegativeTwo,
    oneMinusTwo,
    threeMinusFour,
    threeMinusNegativeFour,
} from "@tests/js/valid/integers/subtraction.mjs"
import { describe, expect, it } from "bun:test"

describe("1 and 2", () => {
    it("1 - 2 == -1", () => {
        expect(oneMinusTwo.valueOf()).toBe(-1)
    })

    it("1 - -2 == 3", () => {
        expect(oneMinusNegativeTwo.valueOf()).toBe(3)
    })

    it("-1 - 2 == -3", () => {
        expect(negativeOneMinusTwo.valueOf()).toBe(-3)
    })

    it("-1 - -2 == 1", () => {
        expect(negativeOneMinusNegativeTwo.valueOf()).toBe(1)
    })
})

describe("3 and 4", () => {
    it("3 - 4 == -1", () => {
        expect(threeMinusFour.valueOf()).toBe(-1)
    })

    it("3 - -4 == 7", () => {
        expect(threeMinusNegativeFour.valueOf()).toBe(7)
    })

    it("-3 - 4 == -7", () => {
        expect(negativeThreeMinusFour.valueOf()).toBe(-7)
    })

    it("-3 - -4 == 1", () => {
        expect(negativeThreeMinusNegativeFour.valueOf()).toBe(1)
    })
})
