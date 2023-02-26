import {
    BnegativeOnePlusNegativeTwo,
    BnegativeOnePlusTwo,
    BnegativeThreePlusFour,
    BnegativeThreePlusNegativeFour,
    BonePlusNegativeTwo,
    BonePlusTwo,
    BthreePlusFour,
    BthreePlusNegativeFour,
} from "@tests/js/valid/integers/addition.mjs"
import { describe, expect, it } from "bun:test"

describe("1 and 2", () => {
    it("1 + 2 == 3", () => {
        expect(BonePlusTwo.valueOf()).toBe(3)
    })

    it("1 + -2 == -1", () => {
        expect(BonePlusNegativeTwo.valueOf()).toBe(-1)
    })

    it("-1 + 2 == 1", () => {
        expect(BnegativeOnePlusTwo.valueOf()).toBe(1)
    })

    it("-1 + -2 == -3", () => {
        expect(BnegativeOnePlusNegativeTwo.valueOf()).toBe(-3)
    })
})

describe("3 and 4", () => {
    it("3 + 4 == 7", () => {
        expect(BthreePlusFour.valueOf()).toBe(7)
    })

    it("3 + -4 == -1", () => {
        expect(BthreePlusNegativeFour.valueOf()).toBe(-1)
    })

    it("-3 + 4 == 1", () => {
        expect(BnegativeThreePlusFour.valueOf()).toBe(1)
    })

    it("-3 + -4 == -7", () => {
        expect(BnegativeThreePlusNegativeFour.valueOf()).toBe(-7)
    })
})
