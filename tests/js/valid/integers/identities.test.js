import {
    fiveDivideOne,
    fiveMinusZero,
    fivePlusZero,
    fivePowerOne,
    fiveTimesOne,
    fourDivideOne,
    fourMinusZero,
    fourPlusZero,
    fourPowerOne,
    fourTimesOne,
    negativeFiveDivideOne,
    negativeFiveMinusZero,
    negativeFivePlusZero,
    negativeFivePowerOne,
    negativeFiveTimesOne,
    negativeFourDivideOne,
    negativeFourMinusZero,
    negativeFourPlusZero,
    negativeFourPowerOne,
    negativeFourTimesOne,
} from "@tests/js/valid/integers/identities.mjs"
import { describe, expect, it } from "bun:test"

describe("addition has identities", () => {
    it("5 + 0 == 5", () => {
        expect(fivePlusZero.valueOf()).toBe(5)
    })

    it("-5 + 0 == -5", () => {
        expect(negativeFivePlusZero.valueOf()).toBe(-5)
    })

    it("4 + 0 = 4", () => {
        expect(fourPlusZero.valueOf()).toBe(4)
    })

    it("-4 + 0 = -4", () => {
        expect(negativeFourPlusZero.valueOf()).toBe(-4)
    })
})

describe("subtraction has identities", () => {
    it("5 - 0 == 5", () => {
        expect(fiveMinusZero.valueOf()).toBe(5)
    })

    it("-5 - 0 == -5", () => {
        expect(negativeFiveMinusZero.valueOf()).toBe(-5)
    })

    it("4 - 0 = 4", () => {
        expect(fourMinusZero.valueOf()).toBe(4)
    })

    it("-4 - 0 = -4", () => {
        expect(negativeFourMinusZero.valueOf()).toBe(-4)
    })
})

describe("multiplication has identities", () => {
    it("5 * 1 == 5", () => {
        expect(fiveTimesOne.valueOf()).toBe(5)
    })

    it("-5 * 1 == -5", () => {
        expect(negativeFiveTimesOne.valueOf()).toBe(-5)
    })

    it("4 * 1 = 4", () => {
        expect(fourTimesOne.valueOf()).toBe(4)
    })

    it("-4 * 1 = -4", () => {
        expect(negativeFourTimesOne.valueOf()).toBe(-4)
    })
})

describe("division has identities", () => {
    it("5 / 1 == 5", () => {
        expect(fiveDivideOne.valueOf()).toBe(5)
    })

    it("-5 / 1 == -5", () => {
        expect(negativeFiveDivideOne.valueOf()).toBe(-5)
    })

    it("4 / 1 = 4", () => {
        expect(fourDivideOne.valueOf()).toBe(4)
    })

    it("-4 / 1 = -4", () => {
        expect(negativeFourDivideOne.valueOf()).toBe(-4)
    })
})

describe("exponentiation has identities", () => {
    it("5 ** 1 == 5", () => {
        expect(fivePowerOne.valueOf()).toBe(5)
    })

    it("-5 ** 1 == -5", () => {
        expect(negativeFivePowerOne.valueOf()).toBe(-5)
    })

    it("4 ** 1 = 4", () => {
        expect(fourPowerOne.valueOf()).toBe(4)
    })

    it("-4 ** 1 = -4", () => {
        expect(negativeFourPowerOne.valueOf()).toBe(-4)
    })
})
