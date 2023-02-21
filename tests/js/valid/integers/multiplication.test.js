import {
    fourTimesZero,
    negativeFourTimesZero,
    negativeOneTimesNegativeTwo,
    negativeOneTimesTwo,
    negativeThreeTimesFour,
    negativeThreeTimesNegativeFour,
    oneTimesNegativeTwo,
    oneTimesTwo,
    threeTimesFour,
    threeTimesNegativeFour,
} from "@tests/js/valid/integers/multiplication.mjs"
import { describe, expect, it } from "bun:test"

describe("1 and 2", () => {
    it("1 * 2 == 2", () => {
        expect(oneTimesTwo.valueOf()).toBe(2)
    })

    it("1 * -2 == -2", () => {
        expect(oneTimesNegativeTwo.valueOf()).toBe(-2)
    })

    it("-1 * 2 == -2", () => {
        expect(negativeOneTimesTwo.valueOf()).toBe(-2)
    })

    it("-1 * -2 == 2", () => {
        expect(negativeOneTimesNegativeTwo.valueOf()).toBe(2)
    })
})

describe("3 and 4", () => {
    it("3 * 4 == 12", () => {
        expect(threeTimesFour.valueOf()).toBe(12)
    })

    it("3 * -4 == -12", () => {
        expect(threeTimesNegativeFour.valueOf()).toBe(-12)
    })

    it("-3 * 4 == -12", () => {
        expect(negativeThreeTimesFour.valueOf()).toBe(-12)
    })

    it("-3 * -4 == 12", () => {
        expect(negativeThreeTimesNegativeFour.valueOf()).toBe(12)
    })
})

describe("multiplying anything by 0 should return 0", () => {
    it("4 * 0 == 0", () => {
        expect(fourTimesZero.valueOf()).toBe(0)
    })

    it("-4 * 0 == 0", () => {
        expect(negativeFourTimesZero.valueOf()).toBe(0)
    })
})
