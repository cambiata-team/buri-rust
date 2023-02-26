import {
    BeightyFiveModEightMinusTwo,
    BeightyFiveModEightPlusTwo,
    BeightyFiveModLParenEightMinusTwoRParen,
    BeightyFiveModLParenEightPlusTwoRParen,
    BlParenEightyFiveModEightRParenMinusTwo,
    BlParenEightyFiveModEightRParenPlusTwo,
    BlParenTwelveDividedThreeRParenMinusOne,
    BlParenTwelveDividedThreeRParenPlusOne,
    BlParenTwoPowerThreeRParenMinusOne,
    BlParenTwoPowerThreeRParenPlusOne,
    BlParenTwoTimesThreeRParenMinusOne,
    BlParenTwoTimesThreeRParenPlusOne,
    BoneMinusTwelveDividedThree,
    BoneMinusTwoPowerThree,
    BoneMinusTwoTimesThree,
    BonePlusLParenTwoTimesThreeRParenPowerLParenSevenMinusFiveRParen,
    BonePlusTwelveDividedThree,
    BonePlusTwoPowerThree,
    BonePlusTwoTimesThree,
    BonePlusTwoTimesThreePowerSevenMinusFive,
    BtwelveDividedLParenThreeMinusOneRParen,
    BtwelveDividedLParenThreePlusOneRParen,
    BtwelveDividedThreeMinusOne,
    BtwelveDividedThreePlusOne,
    BtwoMinusEightyFiveModEight,
    BtwoPlusEightyFiveModEight,
    BtwoPowerLParenThreeMinusOneRParen,
    BtwoPowerLParenThreePlusOneRParen,
    BtwoPowerThreeMinusOne,
    BtwoPowerThreePlusOne,
    BtwoTimesLParenThreeMinusOneRParen,
    BtwoTimesLParenThreePlusOneRParen,
    BtwoTimesThreeMinusOne,
    BtwoTimesThreePlusOne,
} from "@tests/js/valid/integers/order-of-operations.mjs"
import { describe, expect, it } from "bun:test"

describe("raw expressions", () => {
    it("2 * 3 + 1", () => {
        expect(BtwoTimesThreePlusOne.valueOf()).toBe(7)
    })

    it("1 + 2 * 3", () => {
        expect(BonePlusTwoTimesThree.valueOf()).toBe(7)
    })

    it("12 / 3 + 1", () => {
        expect(BtwelveDividedThreePlusOne.valueOf()).toBe(5)
    })

    it("1 + 12 / 3", () => {
        expect(BonePlusTwelveDividedThree.valueOf()).toBe(5)
    })

    it("85 % 8 + 2", () => {
        expect(BeightyFiveModEightPlusTwo.valueOf()).toBe(7)
    })

    it("2 + 85 % 8", () => {
        expect(BtwoPlusEightyFiveModEight.valueOf()).toBe(7)
    })

    it("2 ** 3 + 1", () => {
        expect(BtwoPowerThreePlusOne.valueOf()).toBe(9)
    })

    it("1 + 2 ** 3", () => {
        expect(BonePlusTwoPowerThree.valueOf()).toBe(9)
    })

    it("2 * 3 - 1", () => {
        expect(BtwoTimesThreeMinusOne.valueOf()).toBe(5)
    })

    it("1 - 2 * 3", () => {
        expect(BoneMinusTwoTimesThree.valueOf()).toBe(-5)
    })

    it("12 / 3 - 1", () => {
        expect(BtwelveDividedThreeMinusOne.valueOf()).toBe(3)
    })

    it("1 - 12 / 3", () => {
        expect(BoneMinusTwelveDividedThree.valueOf()).toBe(-3)
    })

    it("85 % 8 - 2", () => {
        expect(BeightyFiveModEightMinusTwo.valueOf()).toBe(3)
    })

    it("2 - 85 % 8", () => {
        expect(BtwoMinusEightyFiveModEight.valueOf()).toBe(-3)
    })

    it("2 ** 3 - 1", () => {
        expect(BtwoPowerThreeMinusOne.valueOf()).toBe(7)
    })

    it("1 - 2 ** 3", () => {
        expect(BoneMinusTwoPowerThree.valueOf()).toBe(-7)
    })
})

describe("with parenthesis", () => {
    it("(2 * 3) + 1", () => {
        expect(BlParenTwoTimesThreeRParenPlusOne.valueOf()).toBe(7)
    })

    it("2 * (3 + 1)", () => {
        expect(BtwoTimesLParenThreePlusOneRParen.valueOf()).toBe(8)
    })

    it("(12 / 3) + 1", () => {
        expect(BlParenTwelveDividedThreeRParenPlusOne.valueOf()).toBe(5)
    })

    it("12 / (3 + 1)", () => {
        expect(BtwelveDividedLParenThreePlusOneRParen.valueOf()).toBe(3)
    })

    it("(85 % 8) + 2", () => {
        expect(BlParenEightyFiveModEightRParenPlusTwo.valueOf()).toBe(7)
    })

    it("85 % (8 + 2)", () => {
        expect(BeightyFiveModLParenEightPlusTwoRParen.valueOf()).toBe(5)
    })

    it("(2 ** 3) + 1", () => {
        expect(BlParenTwoPowerThreeRParenPlusOne.valueOf()).toBe(9)
    })

    it("2 ** (3 + 1)", () => {
        expect(BtwoPowerLParenThreePlusOneRParen.valueOf()).toBe(16)
    })

    it("(2 * 3) - 1", () => {
        expect(BlParenTwoTimesThreeRParenMinusOne.valueOf()).toBe(5)
    })

    it("2 * (3 - 1)", () => {
        expect(BtwoTimesLParenThreeMinusOneRParen.valueOf()).toBe(4)
    })

    it("(12 / 3) - 1", () => {
        expect(BlParenTwelveDividedThreeRParenMinusOne.valueOf()).toBe(3)
    })

    it("12 / (3 - 1)", () => {
        expect(BtwelveDividedLParenThreeMinusOneRParen.valueOf()).toBe(6)
    })

    it("(85 % 8) - 2", () => {
        expect(BlParenEightyFiveModEightRParenMinusTwo.valueOf()).toBe(3)
    })

    it("85 % (8 - 2)", () => {
        expect(BeightyFiveModLParenEightMinusTwoRParen.valueOf()).toBe(1)
    })

    it("(2 ** 3) - 1", () => {
        expect(BlParenTwoPowerThreeRParenMinusOne.valueOf()).toBe(7)
    })

    it("2 ** (3 - 1)", () => {
        expect(BtwoPowerLParenThreeMinusOneRParen.valueOf()).toBe(4)
    })
})

describe("complex expressions", () => {
    it("1 + 2 * 3 ** 7 - 5", () => {
        expect(BonePlusTwoTimesThreePowerSevenMinusFive.valueOf()).toBe(4370)
    })

    it("1 + (2 * 3) ** (7 - 5)", () => {
        expect(
            BonePlusLParenTwoTimesThreeRParenPowerLParenSevenMinusFiveRParen.valueOf()
        ).toBe(37)
    })
})
