import {
    negativeOnePlusNegativeTwo,
    negativeOnePlusTwo,
    negativeThreePlusFour,
    negativeThreePlusNegativeFour,
    onePlusNegativeTwo,
    onePlusTwo,
    threePlusFour,
    threePlusNegativeFour,
} from "@tests/js/valid/integers/addition.mjs"
import { describe, expect, it } from "bun:test"

describe("1 and 2", () => {
    it("1 + 2 == 3", () => {
        expect(onePlusTwo.valueOf()).toBe(3)
    })

    it("1 + -2 == -1", () => {
        expect(onePlusNegativeTwo.valueOf()).toBe(-1)
    })

    it("-1 + 2 == 1", () => {
        expect(negativeOnePlusTwo.valueOf()).toBe(1)
    })

    it("-1 + -2 == -3", () => {
        expect(negativeOnePlusNegativeTwo.valueOf()).toBe(-3)
    })
})

describe("3 and 4", () => {
    it("3 + 4 == 7", () => {
        expect(threePlusFour.valueOf()).toBe(7)
    })

    it("3 + -4 == -1", () => {
        expect(threePlusNegativeFour.valueOf()).toBe(-1)
    })

    it("-3 + 4 == 1", () => {
        expect(negativeThreePlusFour.valueOf()).toBe(1)
    })

    it("-3 + -4 == -7", () => {
        expect(negativeThreePlusNegativeFour.valueOf()).toBe(-7)
    })
})
