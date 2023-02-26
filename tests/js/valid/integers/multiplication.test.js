import {
    BfourTimesZero,
    BnegativeFourTimesZero,
    BnegativeOneTimesNegativeTwo,
    BnegativeOneTimesTwo,
    BnegativeThreeTimesFour,
    BnegativeThreeTimesNegativeFour,
    BoneTimesNegativeTwo,
    BoneTimesTwo,
    BthreeTimesFour,
    BthreeTimesNegativeFour,
} from "@tests/js/valid/integers/multiplication.mjs"
import { describe, expect, it } from "bun:test"

describe("1 and 2", () => {
    it("1 * 2 == 2", () => {
        expect(BoneTimesTwo.valueOf()).toBe(2)
    })

    it("1 * -2 == -2", () => {
        expect(BoneTimesNegativeTwo.valueOf()).toBe(-2)
    })

    it("-1 * 2 == -2", () => {
        expect(BnegativeOneTimesTwo.valueOf()).toBe(-2)
    })

    it("-1 * -2 == 2", () => {
        expect(BnegativeOneTimesNegativeTwo.valueOf()).toBe(2)
    })
})

describe("3 and 4", () => {
    it("3 * 4 == 12", () => {
        expect(BthreeTimesFour.valueOf()).toBe(12)
    })

    it("3 * -4 == -12", () => {
        expect(BthreeTimesNegativeFour.valueOf()).toBe(-12)
    })

    it("-3 * 4 == -12", () => {
        expect(BnegativeThreeTimesFour.valueOf()).toBe(-12)
    })

    it("-3 * -4 == 12", () => {
        expect(BnegativeThreeTimesNegativeFour.valueOf()).toBe(12)
    })
})

describe("multiplying anything by 0 should return 0", () => {
    it("4 * 0 == 0", () => {
        expect(BfourTimesZero.valueOf()).toBe(0)
    })

    it("-4 * 0 == 0", () => {
        expect(BnegativeFourTimesZero.valueOf()).toBe(0)
    })
})
