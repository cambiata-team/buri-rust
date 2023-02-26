import {
    BnegativeOneMinusNegativeTwo,
    BnegativeOneMinusTwo,
    BnegativeThreeMinusFour,
    BnegativeThreeMinusNegativeFour,
    BoneMinusNegativeTwo,
    BoneMinusTwo,
    BthreeMinusFour,
    BthreeMinusNegativeFour,
} from "@tests/js/valid/integers/subtraction.mjs"
import { describe, expect, it } from "bun:test"

describe("1 and 2", () => {
    it("1 - 2 == -1", () => {
        expect(BoneMinusTwo.valueOf()).toBe(-1)
    })

    it("1 - -2 == 3", () => {
        expect(BoneMinusNegativeTwo.valueOf()).toBe(3)
    })

    it("-1 - 2 == -3", () => {
        expect(BnegativeOneMinusTwo.valueOf()).toBe(-3)
    })

    it("-1 - -2 == 1", () => {
        expect(BnegativeOneMinusNegativeTwo.valueOf()).toBe(1)
    })
})

describe("3 and 4", () => {
    it("3 - 4 == -1", () => {
        expect(BthreeMinusFour.valueOf()).toBe(-1)
    })

    it("3 - -4 == 7", () => {
        expect(BthreeMinusNegativeFour.valueOf()).toBe(7)
    })

    it("-3 - 4 == -7", () => {
        expect(BnegativeThreeMinusFour.valueOf()).toBe(-7)
    })

    it("-3 - -4 == 1", () => {
        expect(BnegativeThreeMinusNegativeFour.valueOf()).toBe(1)
    })
})
