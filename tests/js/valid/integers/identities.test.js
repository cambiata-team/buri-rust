import {
    BfiveDivideOne,
    BfiveMinusZero,
    BfivePlusZero,
    BfivePowerOne,
    BfiveTimesOne,
    BfourDivideOne,
    BfourMinusZero,
    BfourPlusZero,
    BfourPowerOne,
    BfourTimesOne,
    BnegativeFiveDivideOne,
    BnegativeFiveMinusZero,
    BnegativeFivePlusZero,
    BnegativeFivePowerOne,
    BnegativeFiveTimesOne,
    BnegativeFourDivideOne,
    BnegativeFourMinusZero,
    BnegativeFourPlusZero,
    BnegativeFourPowerOne,
    BnegativeFourTimesOne,
} from "@tests/js/valid/integers/identities.mjs"
import { describe, expect, it } from "bun:test"

describe("addition has identities", () => {
    it("5 + 0 == 5", () => {
        expect(BfivePlusZero.valueOf()).toBe(5)
    })

    it("-5 + 0 == -5", () => {
        expect(BnegativeFivePlusZero.valueOf()).toBe(-5)
    })

    it("4 + 0 = 4", () => {
        expect(BfourPlusZero.valueOf()).toBe(4)
    })

    it("-4 + 0 = -4", () => {
        expect(BnegativeFourPlusZero.valueOf()).toBe(-4)
    })
})

describe("subtraction has identities", () => {
    it("5 - 0 == 5", () => {
        expect(BfiveMinusZero.valueOf()).toBe(5)
    })

    it("-5 - 0 == -5", () => {
        expect(BnegativeFiveMinusZero.valueOf()).toBe(-5)
    })

    it("4 - 0 = 4", () => {
        expect(BfourMinusZero.valueOf()).toBe(4)
    })

    it("-4 - 0 = -4", () => {
        expect(BnegativeFourMinusZero.valueOf()).toBe(-4)
    })
})

describe("multiplication has identities", () => {
    it("5 * 1 == 5", () => {
        expect(BfiveTimesOne.valueOf()).toBe(5)
    })

    it("-5 * 1 == -5", () => {
        expect(BnegativeFiveTimesOne.valueOf()).toBe(-5)
    })

    it("4 * 1 = 4", () => {
        expect(BfourTimesOne.valueOf()).toBe(4)
    })

    it("-4 * 1 = -4", () => {
        expect(BnegativeFourTimesOne.valueOf()).toBe(-4)
    })
})

describe("division has identities", () => {
    it("5 / 1 == 5", () => {
        expect(BfiveDivideOne.valueOf()).toBe(5)
    })

    it("-5 / 1 == -5", () => {
        expect(BnegativeFiveDivideOne.valueOf()).toBe(-5)
    })

    it("4 / 1 = 4", () => {
        expect(BfourDivideOne.valueOf()).toBe(4)
    })

    it("-4 / 1 = -4", () => {
        expect(BnegativeFourDivideOne.valueOf()).toBe(-4)
    })
})

describe("exponentiation has identities", () => {
    it("5 ** 1 == 5", () => {
        expect(BfivePowerOne.valueOf()).toBe(5)
    })

    it("-5 ** 1 == -5", () => {
        expect(BnegativeFivePowerOne.valueOf()).toBe(-5)
    })

    it("4 ** 1 = 4", () => {
        expect(BfourPowerOne.valueOf()).toBe(4)
    })

    it("-4 ** 1 = -4", () => {
        expect(BnegativeFourPowerOne.valueOf()).toBe(-4)
    })
})
