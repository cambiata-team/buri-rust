import {
    fourModSix,
    oneMinusTwo,
    onePlusTwo,
    oneTimesTwo,
    sixDivideThree,
    sixModFour,
    threeDivideSix,
    threePowerTwo,
    twoMinusOne,
    twoPlusOne,
    twoPowerThree,
    twoTimesOne,
} from "@tests/js/valid/integers/commutativity.mjs"
import { describe, expect, it } from "bun:test"

describe("commutative operators", () => {
    describe("addition", () => {
        it("1 + 2 == 2 + 1", () => {
            expect(onePlusTwo.valueOf()).toBe(twoPlusOne.valueOf())
        })
    })

    describe("multiplication", () => {
        it("1 * 2 == 2 * 1", () => {
            expect(oneTimesTwo.valueOf()).toBe(twoTimesOne.valueOf())
        })
    })
})

describe("non-commutative operators", () => {
    describe("subtraction", () => {
        it("1 - 2 != 2 - 1", () => {
            expect(oneMinusTwo.valueOf()).not.toBe(twoMinusOne.valueOf())
        })
    })

    describe("division", () => {
        it("6 / 3 != 3 / 6", () => {
            expect(sixDivideThree.valueOf()).not.toBe(threeDivideSix.valueOf())
        })
    })

    describe("modulo", () => {
        it("4 % 6 != 6 % 4", () => {
            expect(fourModSix.valueOf()).not.toBe(sixModFour.valueOf())
        })
    })

    describe("power", () => {
        it("2 ** 3 != 3 ** 2", () => {
            expect(twoPowerThree.valueOf()).not.toBe(threePowerTwo.valueOf())
        })
    })
})
