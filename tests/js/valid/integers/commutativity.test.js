import {
    BfourModSix,
    BoneMinusTwo,
    BonePlusTwo,
    BoneTimesTwo,
    BsixDivideThree,
    BsixModFour,
    BthreeDivideSix,
    BthreePowerTwo,
    BtwoMinusOne,
    BtwoPlusOne,
    BtwoPowerThree,
    BtwoTimesOne,
} from "@tests/js/valid/integers/commutativity.mjs"
import { describe, expect, it } from "bun:test"

describe("commutative operators", () => {
    describe("addition", () => {
        it("1 + 2 == 2 + 1", () => {
            expect(BonePlusTwo.valueOf()).toBe(BtwoPlusOne.valueOf())
        })
    })

    describe("multiplication", () => {
        it("1 * 2 == 2 * 1", () => {
            expect(BoneTimesTwo.valueOf()).toBe(BtwoTimesOne.valueOf())
        })
    })
})

describe("non-commutative operators", () => {
    describe("subtraction", () => {
        it("1 - 2 != 2 - 1", () => {
            expect(BoneMinusTwo.valueOf()).not.toBe(BtwoMinusOne.valueOf())
        })
    })

    describe("division", () => {
        it("6 / 3 != 3 / 6", () => {
            expect(BsixDivideThree.valueOf()).not.toBe(BthreeDivideSix.valueOf())
        })
    })

    describe("modulo", () => {
        it("4 % 6 != 6 % 4", () => {
            expect(BfourModSix.valueOf()).not.toBe(BsixModFour.valueOf())
        })
    })

    describe("power", () => {
        it("2 ** 3 != 3 ** 2", () => {
            expect(BtwoPowerThree.valueOf()).not.toBe(BthreePowerTwo.valueOf())
        })
    })
})
